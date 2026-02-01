pub struct EngineResult {
    pub fen: String,
    pub nodes: u64,
    pub time_ms: u64,
    pub nps: u64,
    pub score: String,
    pub bestmove: String,
}

impl EngineResult {
    pub fn new(
        fen: String,
        nodes: u64,
        time_ms: u64,
        nps: u64,
        score: String,
        bestmove: String,
    ) -> Self {
        EngineResult {
            fen,
            nodes,
            time_ms,
            nps,
            score,
            bestmove,
        }
    }
}
