use core::num;

use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player: Player,
}

impl MapBuilder {
    pub fn new(width: i32, height: i32, number_of_rooms: usize) -> Self {
        Self {
            map: Map::new(width, height),
            rooms: Vec::with_capacity(number_of_rooms),
            player: Player::new(0, 0),
        }
    }
    pub fn build(self) -> MapBuilderResult {
        MapBuilderResult {
            map: self.map,
            player: self.player,
        }
    }
}

pub struct MapBuilderResult {
    pub map: Map,
    pub player: Player,
}

pub enum TileType {
    Floor,
    Wall,
}
pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let tiles: Vec<TileType> = (0..(width * height) as usize)
            .map(|_| TileType::Floor)
            .collect();
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn render(&self, draw: &mut DrawBatch) {
        let fg = RGBA::from_f32(1.0, 1.0, 0.0, 0.5);
        for y in 0..self.height {
            for x in 0..self.width {
                draw.target(0);
                if let Some(idx) = self.idx(x, y) {
                    match self.tiles[idx] {
                        TileType::Floor => {
                            draw.set(Point::new(x, y), ColorPair::new(fg, BLACK), to_cp437(','));
                        }
                        TileType::Wall => {
                            draw.set(Point::new(x, y), ColorPair::new(fg, BLACK), to_cp437('#'));
                        }
                    }
                }
            }
        }
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        if x > self.width || y > self.width {
            return None;
        }
        Some((y * self.width) as usize + x as usize)
    }
}
