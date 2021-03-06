use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
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
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.is_floor(point)
    }

    pub fn fill(&mut self, tile: TileType) {
        self.tiles.iter_mut().for_each(|t| *t = tile);
    }

    pub fn carve_room(&mut self, rect: &Rect, tile: TileType) {
        rect.for_each(|Point { x, y }| {
            if let Some(idx) = self.idx(x, y) {
                self.tiles[idx] = tile;
            }
        });
    }

    pub fn carve_tunnel(&mut self, tunnel: &Tunnel, tile: TileType) {
        tunnel.into_iter().for_each(|Point { x, y }| {
            if let Some(idx) = self.idx(x, y) {
                self.tiles[idx] = tile;
            }
        })
    }

    pub fn is_floor(&self, point: Point) -> bool {
        matches!(self.get_tile(point), Some(TileType::Floor))
    }

    pub fn get_tile(&self, Point { x, y }: Point) -> Option<TileType> {
        self.idx(x, y).map(|idx| self.tiles[idx])
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(((y * self.width) as usize) + x as usize)
    }
}
