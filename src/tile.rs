use bracket_lib::prelude::*;
use crate::utils::{make_weighted_vec, ss_idx};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    pub glyph: FontCharType,
    pub bg: RGB,
    pub bg_lit: RGB,
    pub fg: RGB,
    pub fg_lit: RGB,
    pub console: usize,
}
impl Tile {
    pub fn wall() -> Self {
        let weighted: Vec<u16> =
            make_weighted_vec(&[(185, 1), (186, 1), (187, 20), (188, 20), (189, 1)]);
        let mut rng = RandomNumberGenerator::new();

        Tile {
            tile_type: TileType::Wall,
            glyph: *rng.random_slice_entry(&weighted).unwrap(),
            bg: RGB::from_hex("#090012").unwrap(),
            fg: RGB::named(GREY20),
            bg_lit: RGB::from_hex("#2d1e00").unwrap(),
            fg_lit: RGB::named(ORANGE3),
            console: 0,
        }
    }
    pub fn floor() -> Self {
        let dot = ss_idx(0, 256);
        let choices = [
            // small items
            (ss_idx(40, 72), 1),
            (ss_idx(56, 112), 1),
            (ss_idx(64, 112), 1),
            (ss_idx(112, 224), 1),
            (ss_idx(120, 224), 1),
            (ss_idx(112, 232), 1),
            (ss_idx(120, 232), 1),
            // empty
            // (32, 200),
            // (ss_idx(0, 256), 200),
            (dot, 200),
            // (ss_idx(8, 256), 10),

            // grass
            // (ss_idx(112, 112), 1),
            // (ss_idx(120, 112), 1),
            // small grass
            // (ss_idx(80, 112), 1),
            // (ss_idx(88, 112), 1),

            // decorations
            (ss_idx(24, 16), 1) // skeleton

            // borders
            // (ss_idx(72, 128), 1),
            // (ss_idx(80, 128), 1),
            // (ss_idx(88, 128), 1),
            // (ss_idx(72, 136), 1),
            // (ss_idx(88, 136), 1),
            // (ss_idx(72, 144), 1),
            // (ss_idx(80, 144), 1),
            // (ss_idx(88, 144), 1),

            // cracked 1
            // (ss_idx(64, 40), 20),
            // (ss_idx(64, 48), 20),
            // (ss_idx(72, 40), 20),
            // (ss_idx(72, 48), 20),

            // cracked 2
            // (ss_idx(80, 40), 1),
            // (ss_idx(88, 40), 1),
            // (ss_idx(96, 40), 1),

            // cracked 3
            // (ss_idx(80, 48), 1),
            // (ss_idx(88, 48), 1),
            // (ss_idx(96, 48), 1),

            // striped
            // (ss_idx(40, 216), 1),
            // (ss_idx(48, 216), 1),
            ,
        ];
        let weighted = make_weighted_vec(&choices);
        let mut rng = RandomNumberGenerator::new();
        let glyph = *rng.random_slice_entry(&weighted).unwrap();
        Tile {
            tile_type: TileType::Floor,
            glyph,
            bg: RGB::named(BLACK),
            fg: RGB::named(GREY5),
            bg_lit: RGB::named(GREY3),
            fg_lit: RGB::named(GREY15),
            console: 0,
        }
    }
}
