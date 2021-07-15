use bracket_lib::prelude::*;
use glsp::prelude::*;
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

use crate::utils::ss_idx;

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
        let choices = [185, 186, 187, 188, 189, 300];
        let weights = [1, 1, 20, 20, 1];
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        Tile {
            tile_type: TileType::Wall,
            glyph: choices[dist.sample(&mut rng)] as FontCharType,
            bg: RGB::named(BLACK),
            bg_lit: RGB::named(BLACK),
            fg: RGB::named(GREY20),
            fg_lit: RGB::named(ORANGE3),
            console: 0,
        }
    }
    pub fn floor() -> Self {
        let choices = [
            // 35, // skeleton
            88,
            89,
            104,
            105,
            106,
            107,
            108,
            ss_idx(21, 10),
            ss_idx(21, 11),
            ss_idx(22, 10),
            ss_idx(22, 11),
        ];
        // let weights = [100, 1, 100];
        // let mut rng = thread_rng();
        // let dist = WeightedIndex::new(&weights).unwrap();

        let mut rng = RandomNumberGenerator::new();

        Tile {
            tile_type: TileType::Floor,
            // glyph: choices[dist.sample(&mut rng)] as FontCharType,
            glyph: *rng.random_slice_entry(&choices).unwrap(),
            bg: RGB::named(BLACK),
            bg_lit: RGB::named(BLACK),
            fg: RGB::named(GREY5),
            fg_lit: RGB::named(GREY10),
            console: 0,
        }
    }
}
