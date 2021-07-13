// #![windows_subsystem = "windows"]

// bracket_lib::prelude::add_wasm_support!();

mod api;
mod ecs;
mod glsp_interpreter;
mod keycodes;
mod map;
mod tile;

use api::{GlspCommand, KeyPressed};
use bracket_lib::prelude::*;
use glsp::{compile, RGlobal};
use glsp_interpreter::*;
use num_traits::FromPrimitive;

use crate::{ecs::World, keycodes::StrKeyCode, map::Map};

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

struct State {
    interpreter: GlspInterpreter,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut len: usize = 0;
        self.interpreter.runtime.run(|| {
            // Update the ctx:key global
            if let Some(key) = ctx.key {
                // convert VirtualKeyCode to StrKeyCode
                let key: StrKeyCode = FromPrimitive::from_i32(key as i32).unwrap();
                glsp::set_global("ctx:key", key.to_string().to_lowercase())?;
                KeyPressed::borrow_mut().0.replace(key);
            } else {
                glsp::set_global("ctx:key", "")?;
                KeyPressed::borrow_mut().0.take();
            }

            // Call the `(defn main:update)` function
            self.interpreter.call_update();

            // Execute all deferred commands
            let mut queue = api::CommandQueue::borrow_mut();
            len = queue.0.len();
            for command in queue.0.iter() {
                match command {
                    GlspCommand::Cls => ctx.cls(),
                    GlspCommand::SetChar {
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

        ctx.print(0, 0, format!("{:} fps", ctx.fps));
    }
}

embedded_resource!(CURSES_12, "../resources/Unknown-curses-12x12.png");
embedded_resource!(SB_16, "../resources/16x16-sb-ascii.png");

fn main() -> BError {
    link_resource!(CURSES_12, "resources/Unknown-curses-12x12.png");
    link_resource!(SB_16, "resources/16x16-sb-ascii.png");

    let context = BTermBuilder::new()
        .with_title("Roguelike Tutorial")
        .with_dimensions(WIDTH, HEIGHT)
        .with_vsync(false)
        .with_tile_dimensions(16, 16)
        // .with_font("Unknown-curses-12x12.png", 12, 12)
        // .with_simple_console(WIDTH, HEIGHT, "Unknown-curses-12x12.png")
        .with_font("16x16-sb-ascii.png", 16, 16)
        .with_simple_console(WIDTH, HEIGHT, "16x16-sb-ascii.png")
        .build()?;

    let interpreter = GlspInterpreter::new();
    interpreter.runtime.run(|| {
        // Release: bundle the glsp code
        #[cfg(feature = "compiler")]
        let res = glsp::load_compiled(compile!["./game/main.glsp"])?;
        // Dev: dynamically load the code
        #[cfg(not(feature = "compiler"))]
        let res = glsp::load("./game/main.glsp")?;

        // internals
        glsp::add_rglobal(api::CommandQueue::new());
        glsp::add_rglobal(KeyPressed::new());

        // constants
        glsp::bind_global(":width", WIDTH)?;
        glsp::bind_global(":height", HEIGHT)?;

        // api
        glsp::bind_global("ctx:key", "")?;
        glsp::bind_rfn("cls", &api::cls)?;
        glsp::bind_rfn("set", &api::set_char)?;
        glsp::bind_rfn("key?", &api::is_key_pressed)?;
        glsp::bind_rfn("exit", &api::exit)?;
        glsp::bind_rfn("console:log", &console::log::<String>)?;
        Map::bind_map()?;
        api::bind_geometry()?;
        World::bind_world()?;

        // colors
        glsp::bind_rfn("Color", &api::rgb_color)?;

        // rng
        glsp::bind_rfn("RNG", &RandomNumberGenerator::new)?;
        glsp::RClassBuilder::<RandomNumberGenerator>::new()
            .met("roll-dice", &RandomNumberGenerator::roll_dice)
            .met("range", &RandomNumberGenerator::range::<i32>)
            .build();

        // Call the `(defn main:init)` function
        interpreter.call_init();

        glsp::eval(&res, None)?;
        Ok(())
    });
    let gs = State { interpreter };
    main_loop(context, gs)
}

// #[cfg(target_arch = "wasm32")]
// fn start_watcher(x: i32) {
//     // Do nothing
// }

// #[cfg(not(target_arch = "wasm32"))]
// fn start_watcher(x: i32) {
//     // Watch for changes in the gamelisp code
// }
