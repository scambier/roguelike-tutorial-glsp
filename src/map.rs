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
    pub rooms: Vec<Rect>,
}

impl Map {
    pub fn inject_into_runtime() -> GResult<()> {
        glsp::bind_rfn("Map", &Map::new)?;
        glsp::bind_rfn("draw-map", &draw_map)?;
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
            .build();

        glsp::bind_rfn("Rect", &Rect::with_size::<i32>)?;
        glsp::RClassBuilder::<Rect>::new()
            .met("intersect?", &Rect::intersect)
            .met("center", &|rect: &Rect| {
                let center = rect.center();
                vec![center.x, center.y]
            })
            .build();
        Ok(())
    }

    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        let tiles = vec![TileType::Wall; size];
        Map {
            width,
            height,
            tiles,
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
    let tiles = map.tiles.iter();
    let mut x = 0;
    let mut y = 0;
    // let wall = sym("wall").unwrap();
    // let floor = sym("floor").unwrap();
    for tile in tiles {
        // println!("{:?}", tile == wall);
        match tile {
            TileType::Wall => {
                set_char(
                    x,
                    y,
                    '#',
                    &RGB {
                        r: 0.5,
                        g: 0.5,
                        b: 0.5,
                    },
                    &RGB {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    },
                );
            }
            TileType::Floor => {
                set_char(
                    x,
                    y,
                    '.',
                    &RGB {
                        r: 0.2,
                        g: 0.2,
                        b: 0.2,
                    },
                    &RGB {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    },
                );
            }
            _ => (),
        }
        x += 1;
        if x > (map.width - 1) {
            x = 0;
            y += 1;
        }
    }
}

pub fn field_of_view(x: Num, y: Num, range: Num) {}
