mod tunnel;

use bracket_lib::prelude::*;
pub use tunnel::*;

use crate::prelude::{BuildCommandResult, MapBuilder, Progress, TileType, SCREEN_WIDTH};

#[derive(Debug, Clone, Copy)]
pub enum GeneratorCommand {
    FillMap(TileType),
    GenerateRooms {
        num_of_rooms: i32,
        max_room_size: i32,
    },
    PlacePlayerInRoom,
    Tunnel {
        num_of_tunnels: i32,
    },
}

impl GeneratorCommand {
    pub fn generator_text(&self) -> &'static str {
        match self {
            GeneratorCommand::FillMap(_) => "Fill Map",
            GeneratorCommand::GenerateRooms { .. } => "Generate Rooms",
            GeneratorCommand::PlacePlayerInRoom => "Place Player",
            GeneratorCommand::Tunnel { .. } => "Connect Rooms",
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
            } => builder.build_room(*num_of_rooms, *max_room_size, rng),
            GeneratorCommand::PlacePlayerInRoom => builder.place_player(rng),
            GeneratorCommand::Tunnel { .. } => builder.build_tunnels(rng),
        }
    }
}

pub struct GeneraotrRunner {
    pub commands: Vec<GeneratorCommand>,
    pub progress: Option<Progress>,
    pub run_index: usize,
}

impl GeneraotrRunner {
    pub fn new(commands: Vec<GeneratorCommand>) -> Self {
        Self {
            commands,
            run_index: 0,
            progress: None,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.run_index >= self.commands.len()
    }

    fn progress_num(&self) -> i32 {
        if self.run_index >= self.commands.len() {
            return self.max_progress_num();
        }
        self.commands[0..=self.run_index].iter().enumerate().fold(
            0,
            |stuff, (index, cmd)| match cmd {
                GeneratorCommand::FillMap(_) => stuff + 1,
                GeneratorCommand::GenerateRooms { num_of_rooms, .. } => {
                    self.load_progress(index, stuff, num_of_rooms)
                }
                GeneratorCommand::PlacePlayerInRoom => stuff + 1,
                GeneratorCommand::Tunnel { num_of_tunnels } => {
                    self.load_progress(index, stuff, num_of_tunnels)
                }
            },
        )
    }

    fn max_progress_num(&self) -> i32 {
        self.commands.iter().fold(0, |max, cmd| match cmd {
            GeneratorCommand::FillMap(_) => max + 1,
            GeneratorCommand::GenerateRooms { max_room_size, .. } => max + max_room_size,
            GeneratorCommand::PlacePlayerInRoom => max + 1,
            GeneratorCommand::Tunnel { num_of_tunnels } => max + num_of_tunnels,
        })
    }

    fn load_progress(&self, index: usize, stuff: i32, total: &i32) -> i32 {
        if index == self.run_index {
            stuff + self.get_progress()
        } else {
            stuff + total
        }
    }

    fn get_progress(&self) -> i32 {
        if let Some(Progress { current, .. }) = self.progress {
            current as i32
        } else {
            0
        }
    }

    pub fn next(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        if self.is_finished() {
            return;
        }
        let perform = self.commands[self.run_index].perform(builder, rng);
        self.progress = perform.into();
        if let BuildCommandResult::Finished = perform {
            self.run_index += 1;
        }
    }

    pub fn render_progress_bar(&self, draw: &mut DrawBatch) {
        let pos = Rect::with_size(4, 44, 71, 2);
        draw.draw_double_box(pos, ColorPair::new(YELLOW, BLACK));
        draw.bar_horizontal(
            Point::new(pos.x1 + 1, pos.y1 + 1),
            70,
            self.progress_num(),
            self.max_progress_num(),
            ColorPair::new(WHITE, BLACK),
        );
    }

    pub fn render_menu(&self, draw: &mut DrawBatch) {
        let margin = 5;
        let menu_width = 25;
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

            match command {
                GeneratorCommand::Tunnel { .. } | GeneratorCommand::GenerateRooms { .. } => {
                    match self.run_index.cmp(&i) {
                        std::cmp::Ordering::Less => {
                            builder.append("[").append("   ").append("]");
                        }
                        std::cmp::Ordering::Equal => {
                            builder.append("[").fg(YELLOW);
                            match self.progress {
                                Some(progress) => progress.render(builder),
                                None => {
                                    builder.append("   ");
                                }
                            };
                            builder.fg(WHITE).append("]");
                        }
                        std::cmp::Ordering::Greater => {
                            builder.append("[").append("XXX").append("]");
                        }
                    }
                }
                _ => {}
            }

            builder.ln();
        }
    }
}
