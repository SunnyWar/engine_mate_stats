mod analyzer;
mod config;
mod engine_processor;
mod engine_result;
mod fens;
mod uci_engine;

use std::env;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = match config::parse_args_and_config() {
        Some(cfg) => cfg,
        None => return Ok(()),
    };

    let mut fens = match fens::Fens::load_fens() {
        Ok(f) => f,
        Err(e) => {
            log::error!("Failed to load FENs: {e}");
            return Ok(());
        }
    };

    let mut engine = uci_engine::UciEngine::start(&config.engine_path)?;
    let engine_name = engine_processor::initialize_engine(&mut engine, config.threads)?;

    let results = engine_processor::process_fens(
        &mut engine,
        &mut fens,
        config.num_to_analyze,
        config.nodes,
        config.depth,
    )?;

    let mut analyzer = analyzer::Analyzer::new();
    for result in results {
        analyzer.add_result(result);
    }

    // Print the command line used
    let cmdline: String = env::args().collect::<Vec<_>>().join(" ");
    println!("------------------------------------");
    println!("Command line: {}", cmdline);
    println!("Analysis for engine: {}", engine_name);
    analyzer.analyze();
    Ok(())
}
