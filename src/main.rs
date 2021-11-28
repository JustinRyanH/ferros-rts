mod player;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::player::*;
    pub use bracket_lib::prelude::*;
}
use prelude::*;

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

    pub fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        if x > self.width || y > self.width {
            return None;
        }
        Some((y * self.width) as usize + x as usize)
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
}

struct State {
    player: Player,
    map: Map,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(DIMENSION_WIDTH / 2, DIMENSION_HEIGHT / 2),
            map: Map::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        }
    }

    fn render_tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        draw.target(0);
        draw.cls();
        draw.target(1);
        draw.cls();

        self.player.render(&mut draw);
        self.map.render(&mut draw);

        draw.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        self.player.update(ctx);
        self.render_tick(ctx);
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("Ferros RTS")
        .with_dimensions(DIMENSION_WIDTH, DIMENSION_HEIGHT)
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_tile_dimensions(28, 28)
        .with_fps_cap(30.0)
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, State::new())
}
