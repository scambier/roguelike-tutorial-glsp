mod api;
mod glsp_interpreter;
mod keycodes;

use api::GlspCommand;
use glsp::{RClassBuilder, Val};
use glsp_interpreter::*;
use lazy_static::lazy_static;
use rltk::{GameState, RandomNumberGenerator, Rltk, VirtualKeyCode};
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
            }
            None => {
                KEYPRESSED.lock().unwrap().take();
            }
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
            // iterate commands to call rust functions
            match command {
                GlspCommand::Cls => ctx.cls(),
                GlspCommand::Print {
                    x,
                    y,
                    glyph,
                    fg,
                    bg,
                } => ctx.set(*x, *y, *fg, *bg, *glyph),
                GlspCommand::Exit => ctx.quit(),
            }
        }
        QUEUE.lock().unwrap().clear();

        ctx.print(0, 0, ctx.fps);
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
        glsp::bind_rfn("cls", &api::cls)?;
        glsp::bind_rfn("print", &api::print)?;
        glsp::bind_rfn("key?", &api::key_pressed)?;
        glsp::bind_rfn("exit", &api::exit)?;
        glsp::bind_rfn("sized-arr", &api::sized_arr)?;

        // constants
        glsp::bind_global(":width", WIDTH)?;
        glsp::bind_global(":height", HEIGHT)?;

        // colors
        glsp::bind_rfn("Color", &api::rgb_color)?;

        // rng
        RClassBuilder::<RandomNumberGenerator>::new()
            .met("roll-dice", &RandomNumberGenerator::roll_dice)
            .build();
        glsp::bind_rfn("RNG", &RandomNumberGenerator::new)?;

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
