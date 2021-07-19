use crate::utils::{make_weighted_vec, ss_idx};
use bracket_lib::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    pub glyph: FontCharType,
    pub bg_fog: RGB,
    pub bg: RGB,
    pub fg_fog: RGB,
    pub fg: RGB,
    pub console: usize,
}
impl Tile {
    pub fn wall() -> Self {
        let weighted: Vec<u16> =
            make_weighted_vec(&[(185, 1), (186, 1), (187, 20), (188, 20), (189, 1)]);
        let mut rng = RandomNumberGenerator::new();

        let fg = RGB::named(ORANGE3);
        let bg = RGB::from_hex("#c5b291").unwrap() * RGB::named(GREY20);

        Tile {
            tile_type: TileType::Wall,
            glyph: *rng.random_slice_entry(&weighted).unwrap(),
            fg,
            bg,
            fg_fog: fg.to_greyscale().lerp(RGB::named(BLACK), 0.5),
            bg_fog: bg.to_greyscale(),
            console: 0,
        }
    }
    pub fn floor() -> Self {
        let dot = ss_idx(0, 256);
        let choices = [
            // small items
            (ss_idx(40, 72), 1),
            // (ss_idx(56, 112), 1), // double
            // (ss_idx(64, 112), 1), // double
            (ss_idx(112, 224), 1),
            (ss_idx(120, 224), 1),
            (ss_idx(112, 232), 1),
            (ss_idx(120, 232), 1),
            // empty
            (32, 200),
            // (ss_idx(0, 256), 200),
            (dot, 1),
            // (ss_idx(8, 256), 10),

            // grass
            // (ss_idx(112, 112), 1),
            // (ss_idx(120, 112), 1),
            // small grass
            // (ss_idx(80, 112), 1),
            // (ss_idx(88, 112), 1),

            // decorations
            (ss_idx(24, 16), 1), // skeleton

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
        ];
        let weighted = make_weighted_vec(&choices);
        let mut rng = RandomNumberGenerator::new();
        let glyph = *rng.random_slice_entry(&weighted).unwrap();
        let g = rng.range(0.5, 0.6);
        let fg = RGB::from_f32(g * 1.2, g * 1.2, g * 1.2);
        let bg = RGB::from_f32(g, g, g) * RGB::from_hex("#c5b291").unwrap();
        Tile {
            tile_type: TileType::Floor,
            glyph,
            fg,
            bg,
            fg_fog: RGB::named(BLACK),
            bg_fog: bg.to_greyscale().lerp(RGB::named(BLACK), 0.5),
            console: 0,
        }
    }
}
