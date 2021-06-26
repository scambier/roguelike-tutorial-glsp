mod glsp_interpreter;

use glsp::Val;
use glsp_interpreter::*;
use lazy_static::lazy_static;
use rltk::{GameState, Rltk, VirtualKeyCode};
use std::{fs::read_to_string, sync::Mutex};

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

lazy_static! {
    pub static ref QUEUE: Mutex<Vec<GlspCommand>> = Mutex::new(vec![]);
    pub static ref KEYPRESSED: Mutex<Option<VirtualKeyCode>> = Mutex::new(None);
}

struct State {
    interpreter: GlspInterpreter,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        match ctx.key {
            Some(key) => {
                KEYPRESSED.lock().unwrap().replace(key);
            },
            None => {
                KEYPRESSED.lock().unwrap().take();
            },
        }

        self.interpreter.runtime.run(|| {
            let update = match glsp::global::<_, Val>("run") {
                Ok(Val::GFn(update)) => update,
                Ok(val) => {
                    panic!("Invalid run callback:\n{:}", val);
                }
                Err(e) => {
                    panic!("Cannot compile glsp code:\n{:}", e);
                }
            };
            let _: Val = match glsp::call(&update, ()) {
                Ok(val) => val,
                Err(glsp_err) => {
                    panic!("{:}", &glsp_err);
                }
            };

            glsp::gc();
            Ok(())
        });

        for command in QUEUE.lock().unwrap().iter() {
            match command {
                GlspCommand::Cls => ctx.cls(),
                GlspCommand::Print { x, y, s } => ctx.print(*x, *y, s),
            }
        }
        QUEUE.lock().unwrap().clear();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(WIDTH, HEIGHT)
        .unwrap()
        // .with_tile_dimensions(16, 16)
        .with_title("Roguelike Tutorial")
        .build()?;

    let code = match read_to_string("./game/main.glsp") {
        Ok(code) => code,
        Err(e) => panic!("{:?}", e),
    };

    let interpreter = GlspInterpreter::new();
    interpreter.runtime.run(|| {
        // api
        glsp::bind_rfn("cls", &cls)?;
        glsp::bind_rfn("print", &print)?;
        glsp::bind_rfn("key?", &key_pressed)?;

        // constants
        glsp::bind_global(":width", WIDTH)?;
        glsp::bind_global(":height", HEIGHT)?;

        // parse the code
        let vals = glsp::parse_all(&code, Some("game"))?;

        // initial evaluation
        glsp::eval_multi(&vals, None)?;
        Ok(())
    });
    let gs = State { interpreter };
    rltk::main_loop(context, gs)
}

// #[cfg(target_arch = "wasm32")]
// fn start_watcher(x: i32) {
//     // Do nothing
// }

// #[cfg(not(target_arch = "wasm32"))]
// fn start_watcher(x: i32) {
//     // Watch for changes in the gamelisp code
// }
