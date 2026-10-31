//! Integration tests verifying Strata's runtime performance and memory benchmarks.

use strata::benchmarks::BenchmarkHarness;

#[test]
fn test_strata_engine_performance_benchmarks() {
    let report = BenchmarkHarness::run();
    println!("{}", report.summary());

    // Cold start < 8ms (8000 us)
    assert!(
        report.cold_start_duration_us < 8_000,
        "Cold start took {} µs, exceeding 8ms threshold",
        report.cold_start_duration_us
    );

    // 10,000 nodes layout < 50ms (50,000 us)
    assert!(
        report.layout_10k_nodes_duration_us < 50_000,
        "10k-node layout took {} µs, exceeding 50ms threshold",
        report.layout_10k_nodes_duration_us
    );

    // Frame reconcile < 2ms (2000 us)
    assert!(
        report.frame_reconcile_duration_us < 2_000,
        "Frame reconcile took {} µs, exceeding 2ms threshold",
        report.frame_reconcile_duration_us
    );

    // Baseline memory RSS < 50MB (even in unoptimized debug build)
    assert!(
        report.memory_rss_bytes < 50 * 1024 * 1024,
        "Process RSS memory was {} bytes, exceeding 50MB",
        report.memory_rss_bytes
    );

    assert!(report.is_passing());
}
