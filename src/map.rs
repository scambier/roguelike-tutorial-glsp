use bracket_lib::prelude::*;
use glsp::prelude::*;
use std::cmp::{max, min};

use crate::api::*;
use crate::tile::{Tile, *};

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
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
            .met("idx-xy", &Map::idx_xy)
            .met("get-room", &Map::get_room)
            .met("apply-room", &Map::apply_room)
            .met("get-rooms", &Map::get_rooms)
            .met("add-room", &Map::add_room)
            .met("apply-horizontal-tunnel", &Map::apply_horizontal_tunnel)
            .met("apply-vertical-tunnel", &Map::apply_vertical_tunnel)
            .met("is-walkable", &Map::is_walkable)
            .met("fov", &Map::field_of_view_glsp)
            // Visible tiles
            .met("reveal-tile!", &Map::add_tile_to_revealed)
            .met("show-tile!", &Map::add_tile_to_visible)
            .met("visible-tile?", &|map: &Map, idx: usize| -> bool {
                map.visible_tiles[idx]
            })
            .met("clear-visible-tiles!", &|map: &mut Map| {
                map.visible_tiles.iter_mut().for_each(|t| *t = false)
            })
            .met("a*", &|map: &mut Map, start: usize, end: usize| {
                a_star_search(start, end, map)
            })
            .build();

        glsp::RClassBuilder::<NavigationPath>::new()
            .prop_get("success", &|path: &NavigationPath| path.success)
            .prop_get("steps", &|path: &NavigationPath| path.steps.to_vec())
            .build();

        glsp::bind_rfn("draw-map", &draw_map)?;

        Ok(())
    }

    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        let mut tiles = vec![];
        for _ in 0..size {
            tiles.push(Tile::wall());
        }
        Map {
            width,
            height,
            tiles,
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

    fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.width as usize, idx / self.width as usize)
    }

    fn apply_room(&mut self, room: &Rect) {
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = Tile::floor();
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = Tile::floor();
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = Tile::floor();
            }
        }
    }

    // FIXME: merge with is_walkable
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width || y < 1 || y > self.height {
            return false;
        }
        let idx = self.xy_idx(x, y);
        return self.is_walkable(idx);
    }

    // FIXME: merge with is_exit_valid
    fn is_walkable(&self, idx: usize) -> bool {
        self.tiles[idx].tile_type == TileType::Floor
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
        self.tiles[idx].tile_type == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

pub fn draw_map(map: &Map) {
    let mut x = 0;
    let mut y = 0;
    // let wall = sym("wall").unwrap();
    // let floor = sym("floor").unwrap();
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let fg = if map.visible_tiles[idx] {
                tile.fg
            } else {
                tile.fg_fog
            };
            let bg = if map.visible_tiles[idx] {
                tile.bg
            } else {
                tile.bg_fog
            };
            set_char(x, y, tile.glyph, &fg, &bg, tile.console);
        }
        x += 1;
        if x > (map.width - 1) {
            x = 0;
            y += 1;
        }
    }
}
