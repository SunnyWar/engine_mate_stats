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

    for i in 0..n {
        if let Some(fen) = fens.get_next() {
            println!("Sending FEN {}: {}", i + 1, fen);
            let cmd = format!("position fen {}", fen);
            engine.send_command(&cmd)?;
            let go_cmd = format!("go depth {}", depth);
            engine.send_command(&go_cmd)?;

            // Wait for engine to finish (look for 'bestmove')
            loop {
                let line = engine.read_line()?;
                println!("Engine: {}", line);
                if line.starts_with("bestmove") {
                    break;
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}
