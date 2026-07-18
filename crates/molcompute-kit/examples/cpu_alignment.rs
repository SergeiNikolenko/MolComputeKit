use molcompute_kit::core::{align_and_score, AlignmentAtom, AlignmentMode, AtomMapping};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let probe_positions = [
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0, 0.0],
        [0.2, 0.4, 1.0, 0.0],
    ];
    let reference_positions = probe_positions.map(|[x, y, z, _]| [2.0 - y, -1.0 + x, 0.5 + z, 0.0]);
    let probe = probe_positions.map(|position| AlignmentAtom {
        position,
        gaussian_exponent: 0.8,
        gaussian_amplitude: 1.0,
        partial_charge: 0.1,
    });
    let reference = reference_positions.map(|position| AlignmentAtom {
        position,
        gaussian_exponent: 0.8,
        gaussian_amplitude: 1.0,
        partial_charge: 0.1,
    });
    let mapping = (0..probe.len())
        .map(|index| AtomMapping {
            probe_atom: index as u32,
            reference_atom: index as u32,
            weight: 1.0,
        })
        .collect::<Vec<_>>();

    let result = align_and_score(&probe, &reference, &mapping, AlignmentMode::MappedHorn)?;
    println!("rmsd={:.6}", result.scores.rmsd.expect("mapped RMSD"));
    println!("shape_tanimoto={:.6}", result.scores.shape_tanimoto);
    println!(
        "combined_similarity={:.6}",
        result.scores.combined_similarity
    );
    Ok(())
}
