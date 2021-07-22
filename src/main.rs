// #![windows_subsystem = "windows"]

// bracket_lib::prelude::add_wasm_support!();

mod api;
mod ecs;
mod glsp_interpreter;
mod gui;
mod keycodes;
mod map;
mod tile;
mod utils;
mod gamelog;

use api::GlspCommand;
use bracket_lib::prelude::*;
use glsp_interpreter::*;
use num_traits::FromPrimitive;

use crate::{ecs::World, keycodes::StrKeyCode, map::Map};

// lazy_static! {
//     pub static ref SEED: u64 = RandomNumberGenerator::new().rand();
// }

const WIDTH: i32 = 80;
const HEIGHT: i32 = 45;
const BG_COLOR: (u8, u8, u8) = GREY15;

const CONSOLE_BG: usize = 0;
const CONSOLE_CHARS: usize = 1;
const CONSOLE_TEXT: usize = 2;

struct State {
    interpreter: GlspInterpreter,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.interpreter.tick(ctx);
        ctx.set_active_console(CONSOLE_CHARS);
        ctx.print(0, 0, format!("{:} fps", ctx.fps));
    }
}

// embedded_resource!(CURSES_12, "../resources/Unknown-curses-12x12.png");
// embedded_resource!(SB_16, "../resources/16x16-sb-ascii.png");
embedded_resource!(MRMO, "../resources/MRMOTEXT_rexpaint.png");
embedded_resource!(ACORN, "../resources/Acorntileset8x8.png");

fn main() -> BError {
    // link_resource!(CURSES_12, "resources/Unknown-curses-12x12.png");
    // link_resource!(SB_16, "resources/16x16-sb-ascii.png");
    link_resource!(MRMO, "resources/MRMOTEXT_rexpaint.png");
    link_resource!(ACORN, "resources/Acorntileset8x8.png");

    let tile_size = 8;
    let context = BTermBuilder::new()
        .with_title("Roguelike Tutorial")
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(tile_size * 2, tile_size * 2)
        .with_font("MRMOTEXT_rexpaint.png", tile_size, tile_size)
        .with_font("Acorntileset8x8.png", tile_size, tile_size)
        // Console 0 - Bg
        .with_simple_console(WIDTH, HEIGHT, "MRMOTEXT_rexpaint.png")
        // Console 1 - Chars
        .with_sparse_console_no_bg(WIDTH, HEIGHT, "Acorntileset8x8.png")
        // Console 2 - Text
        .with_sparse_console(WIDTH, HEIGHT, "Acorntileset8x8.png")
        // Options
        // .with_automatic_console_resize(true)
        .with_vsync(false)
        .build()?;

    let interpreter = GlspInterpreter::new();
    interpreter.setup();

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
