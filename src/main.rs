mod analyzer;
mod engine_result;
mod fens;
mod uci_engine;
use std::env;

fn main() -> anyhow::Result<()> {
    let default_num_to_analyze = 10;
    let default_depth = 10;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: engine_mate_stats.exe <path_to_engine> [<number_of_positions>] [<depth>]");
        return Ok(());
    }

    let mut fens = fens::Fens::load_fens();

    let engine_path = &args[1];
    let mut engine = uci_engine::UciEngine::start(engine_path)?;

    engine.send_command("uci")?;

    while let Ok(line) = engine.read_line() {
        println!("Engine: {}", line);
        if line == "uciok" {
            break;
        }
    }

    // Get N from args or default
    let n: usize = if args.len() > 2 {
        args[2].parse().unwrap_or(default_num_to_analyze)
    } else {
        default_num_to_analyze
    };

    // Get depth from args or default
    let depth: usize = if args.len() > 3 {
        args[3].parse().unwrap_or(default_depth)
    } else {
        default_depth
    };

    let mut results = Vec::new();

    for i in 0..n {
        if let Some(fen) = fens.get_next() {
            println!("Sending FEN {}: {}", i + 1, fen);
            let cmd = format!("position fen {}", fen);
            engine.send_command(&cmd)?;
            let go_cmd = format!("go depth {}", depth);
            engine.send_command(&go_cmd)?;

            // Variables to collect info
            let mut nodes = 0u64;
            let mut time_ms = 0u64;
            let mut nps = 0u64;
            let mut score = String::new();
            let mut bestmove = String::new();

            // Wait for engine to finish (look for 'bestmove')
            loop {
                let line = engine.read_line()?;
                println!("Engine: {}", line);
                if line.starts_with("info ") {
                    // Try to parse info line for nodes, time, nps, score
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
                        depth as u32,
                    );
                    results.push(result);
                    break;
                }
            }
        } else {
            break;
        }
    }

    // Analyze all results after processing all FENs
    let mut analyzer = analyzer::Analyzer::new();
    for result in results {
        analyzer.add_result(result);
    }
    analyzer.analyze();
    Ok(())
}
