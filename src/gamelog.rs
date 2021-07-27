use glsp::prelude::*;

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

    pub fn bind() -> GResult<()> {
        glsp::add_rglobal(GameLog::new());
        glsp::bind_rfn("log:add", &GameLog::add::<Val>)?;
        glsp::bind_rfn("log:get", &GameLog::get_messages)?;
        Ok(())
    }
}

impl RGlobal for GameLog {}


