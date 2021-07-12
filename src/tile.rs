use bracket_lib::prelude::*;
use glsp::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    pub glyph: char,
    pub bg: RGB,
    pub bg_lit: RGB,
    pub fg: RGB,
    pub fg_lit: RGB,
}
impl Tile {
    pub fn wall() -> Self {
        Tile {
            tile_type: TileType::Wall,
            glyph: '#',
            bg: RGB::named(BLACK),
            bg_lit: RGB::named(BLACK),
            fg: RGB::named(GREY20),
            fg_lit: RGB::named(ORANGE3),
        }
    }
    pub fn floor() -> Self {
        Tile {
            tile_type: TileType::Floor,
            glyph: '.',
            bg: RGB::named(BLACK),
            bg_lit: RGB::named(BLACK),
            fg: RGB::named(GREY10),
            fg_lit: RGB::named(GREY50),
        }
    }
}
