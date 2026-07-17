fn main() {
    if let Err(error) = molcompute_service::run_compute_service() {
        eprintln!("MolComputeKit service failed: {error}");
        std::process::exit(1);
    }
}
