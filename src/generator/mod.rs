use crate::prelude::*;
#[derive(Debug, Clone, Copy)]
pub enum Tunnel {
    Horizontal { x1: i32, x2: i32, y: i32 },
    Vertical { y1: i32, y2: i32, x: i32 },
}

impl Tunnel {
    pub fn horizontal(x1: i32, x2: i32, y: i32) -> Tunnel {
        Tunnel::Horizontal { x1, x2, y }
    }
    pub fn vertical(y1: i32, y2: i32, x: i32) -> Tunnel {
        Tunnel::Vertical { y1, y2, x }
    }

    pub fn render(&self, draw: &mut DrawBatch) {
        self.into_iter().for_each(|Point { x, y }| {
            draw.set(
                Point::new(x, y),
                ColorPair::new(CYAN, BLACK),
                TileType::Floor,
            );
        });
    }
}

impl IntoIterator for Tunnel {
    type Item = Point;
    type IntoIter = PointLine;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Tunnel::Horizontal { x1, x2, y } => {
                let current = x1.min(x2);
                let max = x1.max(x2);
                PointLine {
                    max,
                    current,
                    static_el: y,
                    static_first: false,
                }
            }
            Tunnel::Vertical { y1, y2, x } => {
                let current = y1.min(y2);
                let high = y1.max(y2);
                PointLine {
                    max: high,
                    current,
                    static_el: x,
                    static_first: true,
                }
            }
        }
    }
}
