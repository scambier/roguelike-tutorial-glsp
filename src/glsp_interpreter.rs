use crate::QUEUE;
use glsp::prelude::*;

pub enum GlspCommand {
    Cls,
    Print { x: i32, y: i32, s: String },
}

pub fn cls() {
    QUEUE.lock().unwrap().push(GlspCommand::Cls);
}
pub fn print(x: i32, y: i32, s: String) {
    QUEUE.lock().unwrap().push(GlspCommand::Print { x, y, s })
}

pub struct GlspInterpreter {
    pub runtime: glsp::Runtime,
    code: String,
    // state: &'a State,
}

impl GlspInterpreter {
    pub fn new() -> Self {
        // Sandboxed builder forbids access to filesystem
        let builder = RuntimeBuilder::new().sandboxed(true);
        GlspInterpreter {
            runtime: builder.build(),
            code: String::from(""),
            // state,
        }
    }

    // pub fn start(&mut self, code: String) {
    //     self.code = code;
    //     self.runtime.run(|| {
    //         glsp::bind_rfn("print", &print)?;
    // 		Ok(())
    //     });
    // }
}
