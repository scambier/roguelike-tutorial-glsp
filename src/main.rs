// #![windows_subsystem = "windows"]

// bracket_lib::prelude::add_wasm_support!();

mod api;
mod ecs;
mod gamelog;
mod glsp_interpreter;
mod gui;
mod keycodes;
mod map;
mod tile;
mod utils;

use crate::{ecs::World, keycodes::StrKeyCode, map::Map};
use api::GlspCommand;
use bracket_lib::prelude::*;
use glsp_interpreter::*;
use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use std::{sync::Mutex, time::UNIX_EPOCH};

lazy_static! {
    pub static ref RNG_SEED: Mutex<i32> = Mutex::new(UNIX_EPOCH.elapsed().unwrap().as_secs() as i32);
    pub static ref RNG: Mutex<RandomNumberGenerator> = Mutex::new(RandomNumberGenerator::new());
}

const WIDTH: i32 = 80;
const HEIGHT: i32 = 45;
const BG_COLOR: (u8, u8, u8) = GREY15;

const CONSOLE_BG: usize = 0;
const CONSOLE_NO_BG: usize = 1; // Transparent background
const CONSOLE_UI: usize = 2;
const CONSOLE_MOUSE: usize = 3;

struct State {
    interpreter: GlspInterpreter,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.interpreter.tick(ctx);
        ctx.set_active_console(CONSOLE_NO_BG);
    }
}

embedded_resource!(TILESET, "../resources/tileset_acorn_mrmotext.png");

fn main() -> BError {
    link_resource!(TILESET, "resources/tileset_acorn_mrmotext.png");

    let tile_size = 8;
    let context = BTermBuilder::new()
        .with_title("Roguelike Tutorial")
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(tile_size * 2, tile_size * 2)
        .with_font("tileset_acorn_mrmotext.png", tile_size, tile_size)
        // Console 0 - Bg
        .with_simple_console(WIDTH, HEIGHT, "tileset_acorn_mrmotext.png")
        // Console 1 - Chars
        .with_sparse_console_no_bg(WIDTH, HEIGHT, "tileset_acorn_mrmotext.png")
        // Console 2 - Text
        .with_sparse_console(WIDTH, HEIGHT, "tileset_acorn_mrmotext.png")
        // Console 3 - Mouse overlay
        .with_sparse_console_no_bg(WIDTH, HEIGHT, "tileset_acorn_mrmotext.png")
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
