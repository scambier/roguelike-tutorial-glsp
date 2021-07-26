use glsp::RGlobal;

pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn new() -> Self {
        GameLog { entries: vec![] }
    }
    pub fn add<T: ToString>(&mut self, entry: T) {
        self.entries.push(entry.to_string());
    }
    pub fn get_messages(&self) -> Vec<String> {
        self.entries.to_vec()
    }

    pub fn rglobal_add(entry: &String) {
        GameLog::borrow_mut().add(entry);
    }
}

impl RGlobal for GameLog {}
