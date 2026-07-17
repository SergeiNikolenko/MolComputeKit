use std::cell::{Cell, RefCell};

use burrete_compute_core::{
    evaluate_pm6_with_accelerators, evaluate_rm1_with_prepared_pairs_and_accelerators,
    evaluate_semiempirical, symmetric_eigendecomposition, Rm1Evaluation, SemiempiricalError,
    SemiempiricalMethod, SemiempiricalMolecule, SemiempiricalScfOptions,
};
use burrete_compute_metal::{
    MetalPm6CorrectionBatch, MetalPm6OneCenterFockBatch, MetalTanimotoRuntime,
    Pm6CorrectionMoleculeDescriptor,
};

pub(crate) fn evaluate_semiempirical_molecule(
    runtime: Option<&MetalTanimotoRuntime>,
    molecule: &SemiempiricalMolecule,
    max_memory_bytes: u64,
) -> Result<(Rm1Evaluation, u64), String> {
    let gpu_time_ms = Cell::new(0_u64);
    let gpu_eigensolves = Cell::new(0_usize);
    let previous_fock = RefCell::new(None::<Vec<f64>>);
    let cpu_polishing = Cell::new(false);
    let evaluation = if let Some(runtime) = runtime {
        let diagonalize = |matrix: &[f64], order: usize| {
            if order > 32 {
                return symmetric_eigendecomposition(matrix, order);
            }
            let fock_change = previous_fock
                .borrow()
                .as_ref()
                .map(|previous| {
                    matrix
                        .iter()
                        .zip(previous)
                        .map(|(current, previous)| (current - previous).abs())
                        .fold(0.0_f64, f64::max)
                })
                .unwrap_or(f64::INFINITY);
            previous_fock.replace(Some(matrix.to_vec()));
            if cpu_polishing.get() || fock_change <= 1.0e-5 || gpu_eigensolves.get() >= 32 {
                cpu_polishing.set(true);
                return symmetric_eigendecomposition(matrix, order);
            }
            let dispatch = runtime
                .symmetric_eigen_profiled(matrix, order, max_memory_bytes)
                .map_err(|error| SemiempiricalError::FockBuild(error.to_string()))?;
            gpu_eigensolves.set(gpu_eigensolves.get() + 1);
            gpu_time_ms.set(gpu_time_ms.get() + dispatch.gpu_time_ms);
            Ok((dispatch.eigenvalues, dispatch.eigenvectors))
        };
        if matches!(
            molecule.method,
            SemiempiricalMethod::Pm6 | SemiempiricalMethod::Pm6D | SemiempiricalMethod::Pm6D3H4
        ) {
            evaluate_pm6_with_accelerators(
                molecule,
                SemiempiricalScfOptions::default(),
                |orbital_count, density, pairs| {
                    let dispatch = runtime
                        .contract_pm6_pair_fock_profiled(
                            orbital_count,
                            density,
                            pairs,
                            max_memory_bytes,
                        )
                        .map_err(|error| SemiempiricalError::FockBuild(error.to_string()))?;
                    gpu_time_ms.set(gpu_time_ms.get() + dispatch.gpu_time_ms);
                    Ok(dispatch
                        .contribution_ev
                        .into_iter()
                        .map(f64::from)
                        .collect())
                },
                |density, w_integrals| {
                    let dispatch = runtime
                        .evaluate_pm6_one_center_fock_profiled(
                            MetalPm6OneCenterFockBatch {
                                densities: std::slice::from_ref(density),
                                w_integrals: std::slice::from_ref(w_integrals),
                            },
                            max_memory_bytes,
                        )
                        .map_err(|error| SemiempiricalError::FockBuild(error.to_string()))?;
                    gpu_time_ms.set(gpu_time_ms.get() + dispatch.gpu_time_ms);
                    Ok(dispatch.contributions_ev[0])
                },
                diagonalize,
            )
        } else {
            let prepared = runtime
                .prepare_rm1_pairs_profiled(molecule, max_memory_bytes)
                .map_err(|error| error.to_string())?;
            gpu_time_ms.set(gpu_time_ms.get() + prepared.gpu_time_ms);
            evaluate_rm1_with_prepared_pairs_and_accelerators(
                molecule,
                SemiempiricalScfOptions::default(),
                &prepared.pairs,
                |orbital_count, density, pairs| {
                    let dispatch = runtime
                        .contract_rm1_pair_fock_profiled(
                            orbital_count,
                            density,
                            pairs,
                            max_memory_bytes,
                        )
                        .map_err(|error| SemiempiricalError::FockBuild(error.to_string()))?;
                    gpu_time_ms.set(gpu_time_ms.get() + dispatch.gpu_time_ms);
                    Ok(dispatch
                        .contribution_ev
                        .into_iter()
                        .map(f64::from)
                        .collect())
                },
                diagonalize,
            )
        }
    } else {
        evaluate_semiempirical(molecule, SemiempiricalScfOptions::default())
    }
    .map_err(|error| error.to_string())?;
    if molecule.method == SemiempiricalMethod::Pm6D3H4 {
        if let Some(runtime) = runtime {
            let correction = runtime
                .evaluate_pm6_d3h4_profiled(
                    MetalPm6CorrectionBatch {
                        atoms: &molecule.atoms,
                        molecules: &[Pm6CorrectionMoleculeDescriptor {
                            atom_start: 0,
                            atom_count: molecule.atoms.len(),
                        }],
                    },
                    max_memory_bytes,
                )
                .map_err(|error| error.to_string())?;
            gpu_time_ms.set(gpu_time_ms.get().saturating_add(correction.gpu_time_ms));
        }
    }
    Ok((evaluation, gpu_time_ms.get()))
}
