use glsp::RGlobal;

pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn new() -> Self {
        GameLog { entries: vec![] }
    }
    pub fn add(&mut self, entry: String) {
        self.entries.push(entry);
    }
    pub fn get_messages(&self) -> Vec<String> {
        self.entries.to_vec()
    }
}

impl RGlobal for GameLog {}
