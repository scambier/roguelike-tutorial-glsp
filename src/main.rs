mod api;
mod glsp_interpreter;
mod keycodes;

use api::{GlspCommand, KeyPressed};
use glsp::{RClassBuilder, RGlobal, Val, compile};
use glsp_interpreter::*;
use rltk::{GameState, RandomNumberGenerator, Rltk};

use crate::api::CommandQueue;

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

struct State {
    interpreter: GlspInterpreter,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.interpreter.runtime.run(|| {
            match ctx.key {
                Some(key) => {
                    KeyPressed::borrow_mut().0.replace(key);
                }
                None => {
                    KeyPressed::borrow_mut().0.take();
                }
            }

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

            // Execute all deferred commands
            let mut queue = CommandQueue::borrow_mut();
            for command in queue.0.iter() {
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
            queue.0.clear();

            glsp::gc();
            Ok(())
        });

        ctx.print(0, 0, ctx.fps);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::new()
        .with_title("Roguelike Tutorial")
        .with_dimensions(WIDTH, HEIGHT)
        .with_vsync(false)
        .with_tile_dimensions(12, 12)
        .with_font("../assets/Unknown-curses-12x12.png", 12, 12)
        .with_simple_console(WIDTH, HEIGHT, "../assets/Unknown-curses-12x12.png")
        .build()?;

    let interpreter = GlspInterpreter::new();
    interpreter.runtime.run(|| {
        // globals
        glsp::add_rglobal(CommandQueue::new());
        glsp::add_rglobal(KeyPressed::new());

        // constants
        glsp::bind_global(":width", WIDTH)?;
        glsp::bind_global(":height", HEIGHT)?;

        // api
        glsp::bind_rfn("cls", &api::cls)?;
        glsp::bind_rfn("set", &api::set_char)?;
        glsp::bind_rfn("key?", &api::key_pressed)?;
        glsp::bind_rfn("exit", &api::exit)?;

        // colors
        glsp::bind_rfn("Color", &api::rgb_color)?;

        // rng
        glsp::bind_rfn("RNG", &RandomNumberGenerator::new)?;
        RClassBuilder::<RandomNumberGenerator>::new()
            .met("roll-dice", &RandomNumberGenerator::roll_dice)
            .met("range", &RandomNumberGenerator::range::<i32>)
            .build();

        // Release: bundle the glsp code
        // FIXME: not working
        #[cfg(feature = "compiler")]
        let res = glsp::load_compiled(compile!["./game/main.glsp"])?;
        // Dev: dynamically load the code
        #[cfg(not(feature = "compiler"))]
        let res = glsp::load("./game/main.glsp")?;

        // initial evaluation
        glsp::eval(&res, None)?;
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
