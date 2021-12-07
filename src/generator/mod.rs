mod tunnel;

use bracket_lib::prelude::*;
pub use tunnel::*;

use crate::prelude::*;

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
            GeneratorCommand::FillMap(_) => "Filling Map",
            GeneratorCommand::GenerateRooms { .. } => "Generating Rooms",
            GeneratorCommand::PlacePlayerInRoom => "Placing Player",
            GeneratorCommand::Tunnel { .. } => "Tunneling Between Rooms",
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
            GeneratorCommand::Tunnel { num_of_tunnels } => {
                builder.build_tunnels(*num_of_tunnels, rng)
            }
        }
    }

    pub fn steps(&self) -> usize {
        match self {
            GeneratorCommand::FillMap(_) => 1,
            GeneratorCommand::GenerateRooms { num_of_rooms, .. } => *num_of_rooms as usize,
            GeneratorCommand::PlacePlayerInRoom => 1,
            GeneratorCommand::Tunnel { num_of_tunnels } => *num_of_tunnels as usize,
        }
    }
}

pub struct GeneraotrRunner {
    pub commands: Vec<GeneratorCommand>,
    pub system_progress: Progress,
    pub sub_system_progress: Option<Progress>,
    pub run_index: usize,
}

impl GeneraotrRunner {
    pub fn new(commands: Vec<GeneratorCommand>) -> Self {
        let total = commands.iter().fold(0, |total, cmd| cmd.steps() + total);
        Self {
            commands,
            run_index: 0,
            system_progress: Progress { total, current: 0 },
            sub_system_progress: None,
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
        self.commands.iter().fold(0, |max, cmd| cmd.steps() + max) as i32
    }

    fn load_progress(&self, index: usize, stuff: i32, total: &i32) -> i32 {
        if index == self.run_index {
            stuff + self.get_progress()
        } else {
            stuff + total
        }
    }

    fn get_progress(&self) -> i32 {
        if let Some(Progress { current, .. }) = self.sub_system_progress {
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
        self.sub_system_progress = perform.into();
        if let BuildCommandResult::Finished = perform {
            self.run_index += 1;
        }
    }

    pub fn get_current_command(&self) -> Option<GeneratorCommand> {
        if self.run_index >= self.commands.len() {
            return None;
        }
        Some(self.commands[self.run_index])
    }

    pub fn get_render_text(&self) -> Option<&'static str> {
        self.get_current_command().map(|cmd| cmd.generator_text())
    }
}

impl RenderProgress for GeneraotrRunner {
    fn render_progress(&self, draw: &mut DrawBatch) {
        let y = SCREEN_HEIGHT - 5;
        let pos = Rect::with_size(4, y, 71, 2);

        draw.draw_double_box(pos, ColorPair::new(YELLOW, BLACK));
        if let Some(txt) = self.get_render_text() {
            draw.print_color_centered(y, txt, ColorPair::new(WHITE, BLACK));
        }

        draw.bar_horizontal(
            Point::new(pos.x1 + 1, y + 1),
            70,
            self.progress_num(),
            self.max_progress_num(),
            ColorPair::new(WHITE, BLACK),
        );
    }
}
