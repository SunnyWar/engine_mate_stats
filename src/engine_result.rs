struct EngineResult {
    fen: String,
    nodes: u64,
    time_ms: u64,
    nps: u64,
    score: String,
    bestmove: String,
}

impl EngineResult {
    fn new(
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
