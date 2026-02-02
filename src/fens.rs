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
    pub fn load_fens() -> Self {
        let json = include_str!("./FENs.json");
        let fens_file: FensFile = serde_json::from_str(json).expect("Invalid FENs.json format");
        Fens {
            fens: fens_file.fens,
            index: 0,
        }
    }

    pub fn get_next(&mut self) -> Option<&str> {
        if self.fens.is_empty() {
            return None;
        }
        if self.index >= self.fens.len() {
            return None;
        }
        let fen = &self.fens[self.index];
        self.index += 1;
        Some(fen)
    }
}
