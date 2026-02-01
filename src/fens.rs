use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct FensFile {
    fens: Vec<String>,
}

pub struct Fens {
    fens: Vec<String>,
    index: usize,
}

impl Fens {
    /// Load FENs from the included JSON file at compile time.
    pub fn load_fens() -> Self {
        // The file is included in the binary at compile time
        let json = include_str!("./FENs.json");
        let fens_file: FensFile = serde_json::from_str(json).expect("Invalid FENs.json format");
        Fens {
            fens: fens_file.fens,
            index: 0,
        }
    }

    /// Get the next FEN string, cycling to the start if at the end.
    pub fn get_next(&mut self) -> Option<&str> {
        if self.fens.is_empty() {
            return None;
        }
        let fen = &self.fens[self.index];
        self.index = (self.index + 1) % self.fens.len();
        Some(fen)
    }

    /// Get the FEN string at a specific index.
    pub fn get(&self, index: usize) -> Option<&str> {
        self.fens.get(index).map(|s| s.as_str())
    }
}
