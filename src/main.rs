mod analyzer;
mod engine_processor;
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
    let engine_name = engine_processor::initialize_engine(&mut engine, config.threads)?;

    let results = engine_processor::process_fens(
        &mut engine,
        &mut fens,
        config.num_to_analyze,
        config.nodes,
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
