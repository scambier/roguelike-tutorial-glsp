use glsp::{RGlobal, Val};

pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn new() -> Self {
        GameLog { entries: vec![] }
    }
    pub fn add(&mut self, entry: Val) {
        self.entries.push(entry.to_string());
    }
    pub fn get_messages(&self) -> Vec<String> {
        self.entries.to_vec()
    }
}

impl RGlobal for GameLog {}
