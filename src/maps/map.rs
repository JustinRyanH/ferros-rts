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

    pub fn render(&self, camera: &Camera, draw: &mut DrawBatch) {
        draw.target(0);
        let fg = RGBA::from_f32(1.0, 1.0, 0.0, 0.5);
        let color = ColorPair::new(fg, BLACK);
        for y in 0..self.height {
            for x in 0..self.width {
                draw.target(0);
                if let Some(idx) = self.idx(x, y) {
                    let tile = self.tiles[idx];
                    draw.set(Point::new(x, y) - camera.top_left_corner(), color, tile);
                }
            }
        }
    }

    pub fn get_tile(&self, Point { x, y }: Point) -> Option<TileType> {
        self.idx(x, y).map(|idx| self.tiles[idx])
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
