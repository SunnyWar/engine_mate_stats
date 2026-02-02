use std::env;

pub struct Config {
    pub engine_path: String,
    pub num_to_analyze: usize,
    pub nodes: usize,
    pub threads: usize,
}

pub fn parse_args_and_config() -> Option<Config> {
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
