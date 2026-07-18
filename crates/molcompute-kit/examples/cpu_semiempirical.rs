use molcompute_kit::core::{
    evaluate_rm1, SemiempiricalAtom, SemiempiricalMolecule, SemiempiricalScfOptions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let water = SemiempiricalMolecule::rm1(
        vec![
            SemiempiricalAtom {
                atomic_number: 8,
                position_angstrom: [0.0, 0.0, 0.0],
            },
            SemiempiricalAtom {
                atomic_number: 1,
                position_angstrom: [0.9584, 0.0, 0.0],
            },
            SemiempiricalAtom {
                atomic_number: 1,
                position_angstrom: [-0.2396, 0.9275, 0.0],
            },
        ],
        0,
    )?;
    let result = evaluate_rm1(&water, SemiempiricalScfOptions::default())?;

    println!("status={:?}", result.scf.status);
    println!("iterations={}", result.scf.iterations);
    println!("total_energy_ev={:.9}", result.total_energy_ev);
    println!("atomic_charges={:?}", result.atomic_charges);
    Ok(())
}
