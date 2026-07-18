use molcompute_kit::core::{score_tanimoto_query, Fingerprint2048, TanimotoQueryOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = Fingerprint2048::from_words([u64::MAX; 32]);
    let library = [query, Fingerprint2048::ZERO];
    let counts = score_tanimoto_query(
        &query,
        &library,
        TanimotoQueryOptions::new(8 * 1024 * 1024)?,
    )?;

    for (index, value) in counts.iter().enumerate() {
        println!(
            "record={index} similarity={:.3} intersection={} union={}",
            value.similarity(),
            value.intersection,
            value.union
        );
    }
    Ok(())
}
