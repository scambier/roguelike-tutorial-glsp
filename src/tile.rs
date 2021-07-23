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
        let weighted: Vec<u16> = make_weighted_vec(&[
            (ss_idx(72, 216), 1),
            (ss_idx(80, 216), 1),
            (ss_idx(88, 216), 20),
            (ss_idx(96, 216), 20),
            (ss_idx(104, 216), 1),
        ]);
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
        let choices = [
            // small items
            (ss_idx(40, 200), 1),
            (ss_idx(112, 352), 1),
            (ss_idx(120, 352), 1),
            (ss_idx(112, 360), 1),
            (ss_idx(120, 360), 1),
            (32, 200),           // empty
            (ss_idx(0, 384), 1), // dot
            // decorations
            (ss_idx(24, 144), 1), // skeleton
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
