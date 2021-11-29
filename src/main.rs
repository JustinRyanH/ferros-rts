mod map;
mod player;
mod player_state;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DIMENSION_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DIMENSION_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::player_state::*;
    pub use bracket_lib::prelude::*;
}
use prelude::*;

#[derive(Debug, Clone, Copy)]
enum BuilderState {
    Started,
    Filling,
    Rooms,
    ConnectingRooms,
    PlacingPlayer,
    Finished,
}

impl BuilderState {
    pub fn next(&mut self) {
        *self = match self {
            BuilderState::Started => BuilderState::Filling,
            BuilderState::Filling => BuilderState::Rooms,
            BuilderState::Rooms => BuilderState::ConnectingRooms,
            BuilderState::ConnectingRooms => BuilderState::PlacingPlayer,
            BuilderState::PlacingPlayer => BuilderState::Finished,
            BuilderState::Finished => BuilderState::Finished,
        }
    }

    pub fn has_filled(&self) -> bool {
        !matches!(self, BuilderState::Started | BuilderState::Filling)
    }

    pub fn has_build_rooms(&self) -> bool {
        !matches!(self, BuilderState::Started | BuilderState::Filling | BuilderState::Rooms)
    }

    pub fn has_connected_rooms(&self) -> bool {
        matches!(self, BuilderState::PlacingPlayer | BuilderState::Finished)
    }

    pub fn has_placed_player(&self) -> bool {
        matches!(self, BuilderState::Finished)
    }

    pub  fn is_finished(&self) -> bool {
        matches!(self, BuilderState::Finished)
    }
}

impl Default for BuilderState {
    fn default() -> Self {
        Self::Started
    }
}

pub struct MapBuilderState {
    builder: MapBuilder,
    state: BuilderState,
}

impl MapBuilderState {
    // pub fn render_map(&self, batch: &mut DrawBatch) {}

    fn clear_batch(draw: &mut DrawBatch) {
        draw.target(0);
        draw.cls();
        draw.target(1);
        draw.cls();
    }

    fn submit_batch(ctx: &mut BTerm, draw: &mut DrawBatch) -> BError {
        draw.submit(0)?;
        render_draw_buffer(ctx)
    }

    fn draw_menu(&self, draw: &mut DrawBatch) {
        let margin = 5;
        let menu_width = 20;
        let x = SCREEN_WIDTH - (menu_width + margin);
        let h = 11;
        let modal = Rect::with_size(x, margin, menu_width, h);
        draw.draw_double_box(modal, ColorPair::new(GREY, BLACK));

        let mut buf = TextBuilder::empty();
        let mut block = TextBlock::new(x + 1, margin + 1, menu_width - 1, h - 1);

        let fill_map_text = if self.state.has_filled() { "[X] Fill Map" } else { "[ ] Fill Map"};
        let generate_room_text = if self.state.has_build_rooms() { "[X] Generate Rooms" } else {  "[ ] Generate Rooms" };
        let coonnect_rooms_text = if self.state.has_connected_rooms() { "[X] Connect Rooms" } else {  "[ ] Connect Rooms" };

        let place_player_text = if self.state.has_placed_player() { "[X] Place Player" } else { "[ ] Place Player"  };
        let finished_text = if self.state.is_finished() { "[X] Finished" } else {  "[ ] Finished" };

        buf.fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .centered("Map Building")
            .ln()
            .ln()
            .append(fill_map_text)
            .ln()
            .append(generate_room_text)
            .ln()
            .append(coonnect_rooms_text)
            .ln()
            .append(place_player_text)
            .ln()
            .append(finished_text)
            .ln()
            .ln()
            .fg(RGB::named(RED))
            .centered("Space to Continue")
            .reset();


        block.print(&buf).expect("Text was too big");
        block.render_to_draw_batch(draw);

        match self.state {
            BuilderState::Filling => { draw.set(Point::new(x + 2, margin + 4), ColorPair::new(BLACK, YELLOW), to_cp437('>')); },
            BuilderState::Rooms => { draw.set(Point::new(x + 2, margin + 5), ColorPair::new(BLACK, YELLOW), to_cp437('>')); },
            BuilderState::ConnectingRooms => { draw.set(Point::new(x + 2, margin + 6), ColorPair::new(BLACK, YELLOW), to_cp437('>')); },
            BuilderState::PlacingPlayer => { draw.set(Point::new(x + 2, margin + 7), ColorPair::new(BLACK, YELLOW), to_cp437('>'));  },
            _ => {}
        };
    }
}

impl GameState for MapBuilderState {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        Self::clear_batch(&mut draw);

        self.builder.map.render(&mut draw);

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.state.next();
        }

        self.draw_menu(&mut draw);

        Self::submit_batch(ctx, &mut draw).unwrap();
    }
}

impl Default for MapBuilderState {
    fn default() -> Self {
        Self {
            builder: MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, 10),
            state: BuilderState::default(),
        }
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

    main_loop(context, MapBuilderState::default())
}
