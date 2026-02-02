mod analyzer;
mod engine_result;
mod fens;
mod uci_engine;
use std::env;

struct Config {
    engine_path: String,
    num_to_analyze: usize,
    nodes: usize,
    threads: usize,
}

fn parse_args_and_config() -> Option<Config> {
    let default_num_to_analyze = 10;
    let default_nodes = 10000;
    let default_threads = 8;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: engine_mate_stats.exe <path_to_engine> [<number_of_positions>] [<nodes>]");
        return None;
    }

    let engine_path = args[1].clone();
    let num_to_analyze = if args.len() > 2 {
        args[2].parse().unwrap_or(default_num_to_analyze)
    } else {
        default_num_to_analyze
    };
    let nodes = if args.len() > 3 {
        args[3].parse().unwrap_or(default_nodes)
    } else {
        default_nodes
    };
    let threads = default_threads;

    Some(Config {
        engine_path,
        num_to_analyze,
        nodes,
        threads,
    })
}

fn main() -> anyhow::Result<()> {
    let config = match parse_args_and_config() {
        Some(cfg) => cfg,
        None => return Ok(()),
    };

    let mut fens = fens::Fens::load_fens();

    let mut engine = uci_engine::UciEngine::start(&config.engine_path)?;

    let mut engine_name = String::new();
    engine.send_command("uci")?;

    while let Ok(line) = engine.read_line() {
        println!("Engine: {}", line);
        if line.starts_with("id name ") {
            engine_name = line["id name ".len()..].to_string();
        }

        if line == "uciok" {
            break;
        }
    }

    let results = analyze_fens(
        &mut engine,
        &mut fens,
        config.num_to_analyze,
        config.nodes,
        config.threads,
    )?;

    // Analyze all results after processing all FENs
    let mut analyzer = analyzer::Analyzer::new();
    for result in results {
        analyzer.add_result(result);
    }

    println!("Analysis for engine: {}", engine_name);
    analyzer.analyze();
    Ok(())
}

fn analyze_fens(
    engine: &mut uci_engine::UciEngine,
    fens: &mut fens::Fens,
    n: usize,
    nodes_limit: usize,
    default_threads: usize,
) -> anyhow::Result<Vec<engine_result::EngineResult>> {
    let mut results = Vec::new();
    for i in 0..n {
        if let Some(fen) = fens.get_next() {
            let thread_cmd = format!("setoption name Threads value {}", default_threads);
            engine.send_command(&thread_cmd)?;

            println!("Sending FEN {}: {}", i + 1, fen);
            let cmd = format!("position fen {}", fen);
            engine.send_command(&cmd)?;

            let go_cmd = format!("go nodes {}", nodes_limit);
            engine.send_command(&go_cmd)?;

            // Variables to collect info
            let mut nodes = 0u64;
            let mut time_ms = 0u64;
            let mut nps = 0u64;
            let mut score = String::new();
            let mut bestmove = String::new();
            let mut depth = 0u32;

            // Wait for engine to finish (look for 'bestmove')
            loop {
                let line = engine.read_line()?;
                println!("Engine: {}", line);
                if line.starts_with("info ") {
                    // Try to parse info line for nodes, time, nps, score, depth
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let mut idx = 0;
                    while idx < parts.len() {
                        match parts[idx] {
                            "nodes" => {
                                if idx + 1 < parts.len() {
                                    nodes = parts[idx + 1].parse().unwrap_or(nodes);
                                    idx += 1;
                                }
                            }
                            "time" => {
                                if idx + 1 < parts.len() {
                                    time_ms = parts[idx + 1].parse().unwrap_or(time_ms);
                                    idx += 1;
                                }
                            }
                            "nps" => {
                                if idx + 1 < parts.len() {
                                    nps = parts[idx + 1].parse().unwrap_or(nps);
                                    idx += 1;
                                }
                            }
                            "score" => {
                                if idx + 2 < parts.len() {
                                    score = format!("{} {}", parts[idx + 1], parts[idx + 2]);
                                    idx += 2;
                                }
                            }
                            "depth" => {
                                if idx + 1 < parts.len() {
                                    depth = parts[idx + 1].parse().unwrap_or(depth);
                                    idx += 1;
                                }
                            }
                            _ => {}
                        }
                        idx += 1;
                    }
                } else if line.starts_with("bestmove") {
                    // Parse bestmove
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        bestmove = parts[1].to_string();
                    }
                    // Store result
                    let result = engine_result::EngineResult::new(
                        fen.to_string(),
                        nodes,
                        time_ms,
                        nps,
                        score.clone(),
                        bestmove.clone(),
                        depth,
                    );
                    results.push(result);
                    break;
                }
            }
        } else {
            break;
        }
    }
    Ok(results)
}
