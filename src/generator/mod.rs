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
    pub run_index: usize,
}

impl GeneraotrRunner {
    pub fn new(commands: Vec<GeneratorCommand>) -> Self {
        Self {
            commands,
            run_index: 0,
        }
    }

    pub fn current_step(&self) -> i32 {
        if self.run_index >= self.commands.len() {
            return self.total_steps();
        }
        (0..=self.run_index).fold(0, |current, index| {
            current + self.get_subsystem_current_progress(index)
        })
    }

    pub fn total_steps(&self) -> i32 {
        self.commands
            .iter()
            .fold(0, |total, cmd| cmd.steps() as i32 + total)
    }

    fn get_subsystem_current_progress(&self, index: usize) -> i32 {
        if index != self.run_index {
            return self.commands[index].steps() as i32;
        }
        0
    }

    pub fn next(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        if self.is_finished() {
            builder.finished = true;
            return;
        }
        let perform = self.commands[self.run_index].perform(builder, rng);
        if let BuildCommandResult::Finished = perform {
            self.run_index += 1;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.run_index >= self.commands.len()
    }

    pub fn get_current_command(&self) -> Option<GeneratorCommand> {
        if self.is_finished() {
            return None;
        }
        Some(self.commands[self.run_index])
    }

    pub fn get_render_text(&self) -> Option<&'static str> {
        self.get_current_command().map(|cmd| cmd.generator_text())
    }
}

impl Default for GeneraotrRunner {
    fn default() -> Self {
        let num_of_rooms = 10;
        let commands = vec![
            GeneratorCommand::FillMap(TileType::Wall),
            GeneratorCommand::GenerateRooms {
                num_of_rooms,
                max_room_size: 10,
            },
            GeneratorCommand::Tunnel {
                num_of_tunnels: (num_of_rooms * 2) - 2,
            },
            GeneratorCommand::PlacePlayerInRoom,
        ];
        Self::new(commands)
    }
}
