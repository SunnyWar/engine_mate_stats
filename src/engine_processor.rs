use crate::engine_result::EngineResult;
use crate::fens::Fens;
use crate::uci_engine::UciEngine;
use anyhow::Result;

pub fn process_fens(
    engine: &mut UciEngine,
    fens: &mut Fens,
    n: usize,
    nodes_limit: usize,
    default_threads: usize,
) -> Result<Vec<EngineResult>> {
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
                    let result = EngineResult::new(
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
