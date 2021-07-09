use bracket_lib::prelude::*;
use glsp::prelude::*;
use std::cmp::{max, min};

use crate::api::set_char;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
}

impl TileType {
    pub fn from(symbol: Sym) {
        let wall = sym("wall").unwrap();
        let floor = sym("floor").unwrap();
        match symbol {
            _ if symbol == wall => TileType::Wall,
            _ if symbol == floor => TileType::Floor,
            _ => TileType::Wall,
        };
    }
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub rooms: Vec<Rect>,
}

impl Map {
    pub fn bind_map() -> GResult<()> {
        glsp::bind_rfn("Map", &Map::new)?;
        glsp::RClassBuilder::<Map>::new()
            .prop_get("width", &Map::get_width)
            .prop_get("height", &Map::get_height)
            .met("init", &Map::new)
            .met("xy-idx", &Map::xy_idx)
            .met("get-room", &Map::get_room)
            .met("apply-room", &Map::apply_room)
            .met("get-rooms", &Map::get_rooms)
            .met("add-room", &Map::add_room)
            .met("apply-horizontal-tunnel", &Map::apply_horizontal_tunnel)
            .met("apply-vertical-tunnel", &Map::apply_vertical_tunnel)
            .met("is-walkable", &Map::is_walkable)
            .met("fov", &Map::field_of_view_glsp)
            .met("reveal-tile!", &Map::add_tile_to_revealed)
            .met("show-tile!", &Map::add_tile_to_visible)
            .met("clear-visible-tiles!", &|map: &mut Map| {
                map.visible_tiles.iter_mut().for_each(|t| *t = false)
            })
            .build();

        glsp::bind_rfn("draw-map", &draw_map)?;

        Ok(())
    }

    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Map {
            width,
            height,
            tiles: vec![TileType::Wall; size],
            revealed_tiles: vec![false; size],
            visible_tiles: vec![false; size],
            rooms: vec![],
        }
    }

    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn get_room(&self, id: usize) -> Rect {
        self.rooms[id]
    }
    fn get_rooms(&self) -> Vec<Rect> {
        self.rooms.to_vec()
    }
    fn add_room(&mut self, room: &Rect) {
        self.rooms.push(*room)
    }

    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn apply_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn is_walkable(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Floor
    }

    fn field_of_view_glsp(&self, x: i32, y: i32, range: i32) -> Vec<Point> {
        field_of_view(Point { x, y }, range, self)
    }

    fn add_tile_to_revealed(&mut self, idx: usize) {
        self.revealed_tiles[idx] = true;
    }

    fn add_tile_to_visible(&mut self, idx: usize) {
        self.visible_tiles[idx] = true;
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> bracket_lib::prelude::Point {
        Point {
            x: self.width,
            y: self.height,
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

pub fn draw_map(map: &Map) {
    let mut x = 0;
    let mut y = 0;
    // let wall = sym("wall").unwrap();
    // let floor = sym("floor").unwrap();
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            // println!("{:?}", tile == wall);
            match tile {
                TileType::Wall => {
                    let col = if map.visible_tiles[idx] {
                        RGB::named(LIGHT_BLUE)
                    } else {
                        RGB::named(GREY50)
                    };
                    set_char(x, y, '#', &col, &RGB::named(BLACK));
                }
                TileType::Floor => {
                    let col = if map.visible_tiles[idx] {
                        RGB::named(GREY50)
                    } else {
                        RGB::named(GREY10)
                    };
                    set_char(x, y, '.', &col, &RGB::named(BLACK));
                }
                _ => (),
            }
        }
        x += 1;
        if x > (map.width - 1) {
            x = 0;
            y += 1;
        }
    }
}
