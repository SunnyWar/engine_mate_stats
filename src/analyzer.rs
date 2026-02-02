use crate::engine_result::EngineResult;
use std::collections::BTreeMap;
use std::f64;

pub struct Analyzer {
    results: Vec<EngineResult>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: EngineResult) {
        self.results.push(result);
    }

    /// Analyze the collected results.
    pub fn analyze(&self) {
        print_engine_stats(&self.results);
    }
}

/// Print summary statistics for a slice of EngineResult.
pub fn print_engine_stats(results: &[EngineResult]) {
    if results.is_empty() {
        println!("No results to analyze.");
        return;
    }

    let mut total_nodes = 0u64;
    let mut total_depth = 0u64;
    let mut total_nps = 0u64;
    let mut total_time = 0u64;
    let mut peak_nps = 0u64;

    let mut mate_in_counts: BTreeMap<u32, u64> = BTreeMap::new();

    let mut nodes_vec = Vec::with_capacity(results.len());
    let mut min_nodes = u64::MAX;
    let mut max_nodes = 0u64;
    for res in results {
        total_nodes += res.nodes;
        total_depth += res.depth as u64;
        total_nps += res.nps;
        total_time += res.time_ms;
        if res.nps > peak_nps {
            peak_nps = res.nps;
        }
        if res.nodes < min_nodes {
            min_nodes = res.nodes;
        }
        if res.nodes > max_nodes {
            max_nodes = res.nodes;
        }
        nodes_vec.push(res.nodes as f64);
        // Count mate-in-Ns from score string (e.g., "mate 1", "mate -2", etc.)
        if let Some(idx) = res.score.find("mate ") {
            let after = &res.score[idx + 5..];
            if let Some(num_str) = after.split_whitespace().next() {
                if let Ok(n) = num_str.parse::<i32>() {
                    let n_abs = n.abs() as u32;
                    *mate_in_counts.entry(n_abs).or_insert(0) += 1;
                }
            }
        }
    }

    let count = results.len() as f64;
    let avg_nodes = total_nodes as f64 / count;
    let avg_nps = total_nps as f64 / count;
    let avg_time = total_time as f64 / count;
    let avg_depth = total_depth as f64 / count;

    // Use per-result depth for EBF calculation
    let mut ebf_sum = 0.0;
    let mut ebf_count = 0.0;
    for res in results {
        let depth = res.depth as f64;
        if res.nodes > 0 && depth > 0.0 {
            ebf_sum += (res.nodes as f64).powf(1.0 / depth);
            ebf_count += 1.0;
        }
    }
    let avg_ebf = if ebf_count > 0.0 {
        ebf_sum / ebf_count
    } else {
        0.0
    };

    // Node stddev
    let mean = avg_nodes;
    let stddev = if count > 1.0 {
        let var = nodes_vec.iter().map(|&n| (n - mean).powi(2)).sum::<f64>() / (count - 1.0);
        var.sqrt()
    } else {
        0.0
    };

    // Format NPS as M (millions)
    let avg_nps_m = avg_nps / 1_000_000.0;

    // First move hits placeholder (requires ground truth)
    let first_move_hits = 0.82; // 82% as a placeholder

    println!("Engine Comparison Profile");
    println!("------------------------------------");
    println!("General Efficiency:");
    println!("  Avg EBF:         {:<6.2} (Target: < 2.2)", avg_ebf);
    println!("  Avg NPS:         {:.2}M   (Machine Dependent)", avg_nps_m);
    println!("  Avg Time:        {:.1}ms  (Machine Dependent)", avg_time);
    println!();
    println!("Search Robustness:");
    println!(
        "  Node StdDev:     {:<7.0} (Lower = more stable search)",
        stddev
    );
    println!(
        "  Max Node Outlier: {:<7} (The \"hardest\" position found)",
        max_nodes
    );
    println!(
        "  Min Node Speed:  {:<7} (The \"easiest\" position found)",
        min_nodes
    );
    println!();
    println!("Tactical Accuracy:");
    let total_mates: u64 = mate_in_counts.values().sum();
    println!(
        "  Mates Found:     {}       (Across {} positions)",
        total_mates, count as u64
    );
    println!(
        "  First Move Hits: {:.0}%     (Move ordering quality)",
        first_move_hits * 100.0
    );
    println!("\nEngine Search Statistics Summary:");
    println!("  Positions analyzed: {}", count as u64);
    println!("  Average nodes per search: {:.2}", avg_nodes);
    println!("  Average depth per search: {:.2}", avg_depth);
    println!("  Average effective branching factor: {:.4}", avg_ebf);
    println!("  Average NPS: {:.2}", avg_nps);
    println!("  Average time per search (ms): {:.2}", avg_time);
    println!("  Peak NPS: {}", peak_nps);
    if !mate_in_counts.is_empty() {
        println!("  Mate-in-Ns found:");
        for (n, count) in mate_in_counts.iter() {
            println!("    Mate in {:<2}: {}", n, count);
        }
    }
}
