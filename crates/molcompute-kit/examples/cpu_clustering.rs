use std::num::NonZeroUsize;

use molcompute_kit::{
    core::{
        build_tanimoto_graph, butina_clusters, ButinaOptions, Fingerprint2048, GraphBuildOptions,
    },
    protocol::{ResourceLimits, SimilarityCutoff},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fingerprint = Fingerprint2048::from_words([u64::MAX; 32]);
    let fingerprints = [fingerprint, fingerprint, Fingerprint2048::ZERO];
    let limits = ResourceLimits {
        max_edges: 16,
        max_memory_bytes: 16 * 1024 * 1024,
        max_dispatch_ms: 2_000,
    };

    let graph = build_tanimoto_graph(
        &fingerprints,
        SimilarityCutoff {
            numerator: 1,
            denominator: 2,
        },
        GraphBuildOptions::from_resource_limits(
            NonZeroUsize::new(2).expect("tile size is non-zero"),
            &limits,
        )?,
    )?;
    let clusters = butina_clusters(&graph, ButinaOptions::from_resource_limits(&limits)?)?;

    println!("vertices={}", graph.vertex_count());
    println!("undirected_edges={}", graph.undirected_edge_count());
    println!("clusters={clusters:?}");
    Ok(())
}
