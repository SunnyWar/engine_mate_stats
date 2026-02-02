use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Path to the UCI engine binary
    #[arg(long, short = 'e')]
    pub engine_path: String,

    /// Number of positions to analyze
    #[arg(long, short = 'p', default_value_t = 10)]
    pub num_to_analyze: usize,

    /// Nodes limit per position (mutually exclusive with depth)
    #[arg(long, short = 'n', required = false, conflicts_with = "depth")]
    pub nodes: Option<usize>,

    /// Depth limit per position (mutually exclusive with nodes)
    #[arg(long, short = 'd', required = false, conflicts_with = "nodes")]
    pub depth: Option<usize>,

    /// Threads for engine
    #[arg(long, short, default_value_t = 8)]
    pub threads: usize,
}

pub fn parse_args_and_config() -> Option<Config> {
    let config = Config::try_parse();
    match config {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            log::error!("Argument parsing error: {e}");
            let _ = e.print();
            None
        }
    }
}
