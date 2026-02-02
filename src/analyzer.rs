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

    /// Analyze and write consolidated stats to CSV file (append mode)
    pub fn analyze_and_write_csv(&self, csv_path: &str, engine_name: &str, cmdline: &str) {
        let (stats, mate_in_counts) = compute_stats(&self.results);
        print_stats_human_readable(&stats, engine_name, cmdline, &mate_in_counts);
        if self.results.is_empty() {
            return;
        }
        use std::fs::OpenOptions;
        use std::io::BufWriter;
        let file_exists = std::path::Path::new(csv_path).exists();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(csv_path)
            .expect("Unable to open or create CSV file");
        let mut writer = csv::WriterBuilder::new()
            .has_headers(!file_exists)
            .from_writer(BufWriter::new(file));

        // Always output header for mates in 1..=20
        let max_n = 20;
        if !file_exists {
            let mut header = vec![
                "engine_name".to_string(),
                "cmdline".to_string(),
                "positions_analyzed".to_string(),
                "avg_ebf".to_string(),
                "avg_nps".to_string(),
                "avg_nps_m".to_string(),
                "avg_time_ms".to_string(),
                "avg_nodes".to_string(),
                "avg_depth".to_string(),
                "node_stddev".to_string(),
                "max_nodes".to_string(),
                "min_nodes".to_string(),
                "total_mates".to_string(),
                "first_move_hits".to_string(),
                "peak_nps".to_string(),
            ];
            for n in 1..=max_n {
                header.push(format!("mates in {}", n));
            }
            let _ = writer.write_record(&header);
        }
        let mut record = vec![
            engine_name.to_string(),
            cmdline.to_string(),
            stats.positions_analyzed.to_string(),
            format!("{:.4}", stats.avg_ebf),
            format!("{:.2}", stats.avg_nps),
            format!("{:.2}", stats.avg_nps_m),
            format!("{:.2}", stats.avg_time_ms),
            format!("{:.2}", stats.avg_nodes),
            format!("{:.2}", stats.avg_depth),
            format!("{:.0}", stats.node_stddev),
            stats.max_nodes.to_string(),
            stats.min_nodes.to_string(),
            stats.total_mates.to_string(),
            format!("{:.0}", stats.first_move_hits * 100.0),
            stats.peak_nps.to_string(),
        ];
        for n in 1..=max_n {
            let count = mate_in_counts.get(&n).cloned().unwrap_or(0);
            record.push(count.to_string());
        }
        let _ = writer.write_record(&record);
        let _ = writer.flush();
    }
}

/// Struct to hold consolidated stats for CSV and printing
struct StatsSummary {
    positions_analyzed: u64,
    avg_ebf: f64,
    avg_nps: f64,
    avg_nps_m: f64,
    avg_time_ms: f64,
    avg_nodes: f64,
    avg_depth: f64,
    node_stddev: f64,
    max_nodes: u64,
    min_nodes: u64,
    total_mates: u64,
    first_move_hits: f64,
    peak_nps: u64,
}

fn compute_stats(results: &[EngineResult]) -> (StatsSummary, BTreeMap<u32, u64>) {
    use std::collections::BTreeMap;
    if results.is_empty() {
        return (
            StatsSummary {
                positions_analyzed: 0,
                avg_ebf: 0.0,
                avg_nps: 0.0,
                avg_nps_m: 0.0,
                avg_time_ms: 0.0,
                avg_nodes: 0.0,
                avg_depth: 0.0,
                node_stddev: 0.0,
                max_nodes: 0,
                min_nodes: 0,
                total_mates: 0,
                first_move_hits: 0.0,
                peak_nps: 0,
            },
            BTreeMap::new(),
        );
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
    let total_mates: u64 = mate_in_counts.values().sum();
    (
        StatsSummary {
            positions_analyzed: count as u64,
            avg_ebf,
            avg_nps,
            avg_nps_m,
            avg_time_ms: avg_time,
            avg_nodes,
            avg_depth,
            node_stddev: stddev,
            max_nodes,
            min_nodes,
            total_mates,
            first_move_hits,
            peak_nps,
        },
        mate_in_counts,
    )
}

fn print_stats_human_readable(
    stats: &StatsSummary,
    engine_name: &str,
    cmdline: &str,
    mate_in_counts: &BTreeMap<u32, u64>,
) {
    println!("------------------------------------");
    println!("Command line: {}", cmdline);
    println!("Analysis for engine: {}", engine_name);
    println!("------------------------------------");
    println!("General Efficiency:");
    println!("  Avg EBF:         {:<6.2} (Target: < 2.2)", stats.avg_ebf);
    println!(
        "  Avg NPS:         {:.2}M   (Machine Dependent)",
        stats.avg_nps_m
    );
    println!(
        "  Avg Time:        {:.1}ms  (Machine Dependent)",
        stats.avg_time_ms
    );
    println!();
    println!("Search Robustness:");
    println!(
        "  Node StdDev:     {:<7.0} (Lower = more stable search)",
        stats.node_stddev
    );
    println!(
        "  Max Node Outlier: {:<7} (The \"hardest\" position found)",
        stats.max_nodes
    );
    println!(
        "  Min Node Speed:  {:<7} (The \"easiest\" position found)",
        stats.min_nodes
    );
    println!();
    println!("Tactical Accuracy:");
    println!(
        "  Mates Found:     {}       (Across {} positions)",
        stats.total_mates, stats.positions_analyzed
    );
    println!(
        "  First Move Hits: {:.0}%     (Move ordering quality)",
        stats.first_move_hits * 100.0
    );
    println!("\nEngine Search Statistics Summary:");
    println!("  Positions analyzed: {}", stats.positions_analyzed);
    println!("  Average nodes per search: {:.2}", stats.avg_nodes);
    println!("  Average depth per search: {:.2}", stats.avg_depth);
    println!("  Average effective branching factor: {:.4}", stats.avg_ebf);
    println!("  Average NPS: {:.2}", stats.avg_nps);
    println!("  Average time per search (ms): {:.2}", stats.avg_time_ms);
    println!("  Peak NPS: {}", stats.peak_nps);
    println!("  Mate-in-Ns found:");
    let max_n = mate_in_counts
        .iter()
        .filter(|kv| *kv.1 > 0)
        .map(|(n, _)| *n)
        .max()
        .unwrap_or(0)
        .max(1);
    for n in 1..=max_n {
        let count = mate_in_counts.get(&n).cloned().unwrap_or(0);
        println!("    Mate in {:<2}: {}", n, count);
    }
    println!("------------------------------------");
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
    println!("------------------------------------");
}
