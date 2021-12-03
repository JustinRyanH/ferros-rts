mod tunnel;

use bracket_lib::prelude::*;
pub use tunnel::*;

use crate::prelude::{BuildCommandResult, MapBuilder, TileType, SCREEN_WIDTH};

#[derive(Debug, Clone, Copy)]
pub enum GeneratorCommand {
    FillMap(TileType),
    GenerateRooms {
        num_of_rooms: i32,
        max_room_size: i32,
    },
    PlacePlayerInRoom,
    Tunnel,
}

impl GeneratorCommand {
    pub fn generator_text(&self) -> &'static str {
        match self {
            GeneratorCommand::FillMap(_) => "Fill Map",
            GeneratorCommand::GenerateRooms { .. } => "Generate Rooms",
            GeneratorCommand::PlacePlayerInRoom => "Place Player",
            GeneratorCommand::Tunnel => "Connect Rooms",
        }
    }

    pub fn perform(
        &self,
        builder: &mut MapBuilder,
        rng: &mut RandomNumberGenerator,
    ) -> BuildCommandResult {
        match self {
            GeneratorCommand::FillMap(tile) => builder.fill(tile),
            GeneratorCommand::GenerateRooms {
                num_of_rooms,
                max_room_size,
            } => {
                while let BuildCommandResult::NotFinished =
                    builder.build_room(*num_of_rooms, *max_room_size, rng)
                {}
                BuildCommandResult::Finished
            }
            GeneratorCommand::PlacePlayerInRoom => builder.place_player(rng),
            GeneratorCommand::Tunnel => builder.build_tunnels(rng),
        }
    }
}

pub struct GeneraotrRunner {
    pub commands: Vec<GeneratorCommand>,
    pub run_index: usize,
}

impl GeneraotrRunner {
    pub fn new(commands: Vec<GeneratorCommand>) -> Self {
        Self {
            commands,
            run_index: 0,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.run_index >= self.commands.len()
    }

    pub fn next(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        if self.is_finished() {
            return;
        }
        self.commands[self.run_index].perform(builder, rng);
        self.run_index += 1;
    }

    pub fn render_menu(&self, draw: &mut DrawBatch) {
        let margin = 5;
        let menu_width = 20;
        let x = SCREEN_WIDTH - (menu_width + margin);
        let h = 7 + self.commands.len() as i32;
        let modal = Rect::with_size(x, margin, menu_width, h);
        draw.draw_double_box(modal, ColorPair::new(GREY, BLACK));

        let mut block = TextBlock::new(x + 1, margin + 1, menu_width - 1, h - 1);
        let mut buf = TextBuilder::empty();
        buf.fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .centered("Map Generator")
            .ln()
            .ln();
        self.display_commands(&mut buf);
        buf.ln()
            .fg(RGB::named(RED))
            .centered("Space to Continue")
            .reset();

        block.print(&buf).expect("Text was too big");
        block.render_to_draw_batch(draw);
    }

    pub fn display_commands(&self, builder: &mut TextBuilder) {
        for (i, command) in self.commands.iter().enumerate() {
            match i.cmp(&self.run_index) {
                std::cmp::Ordering::Less => builder.append("[X]"),
                std::cmp::Ordering::Equal => builder
                    .append("[")
                    .fg(YELLOW)
                    .append(">")
                    .fg(WHITE)
                    .append("]"),
                std::cmp::Ordering::Greater => builder.append("[ ]"),
            };
            builder.append(" ").append(command.generator_text());
            builder.ln();
        }
    }
}
