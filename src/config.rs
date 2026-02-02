use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Path to the UCI engine binary
    #[arg(long, short)]
    pub engine_path: String,

    /// Number of positions to analyze
    #[arg(long, short = 'p', default_value_t = 10)]
    pub num_to_analyze: usize,

    /// Nodes limit per position
    #[arg(long, short = 'n', default_value_t = 10000)]
    pub nodes: usize,

    /// Threads for engine
    #[arg(long, short, default_value_t = 8)]
    pub threads: usize,
}

pub fn parse_args_and_config() -> Option<Config> {
    let config = Config::try_parse();
    match config {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            e.print().expect("Failed to print clap error");
            None
        }
    }
}
