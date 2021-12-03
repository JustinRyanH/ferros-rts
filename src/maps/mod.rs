mod map;

use crate::prelude::*;
pub use map::*;

#[derive(Clone, Copy, Debug)]
pub enum BuilderState {
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
        !matches!(
            self,
            BuilderState::Started | BuilderState::Filling | BuilderState::Rooms
        )
    }

    pub fn has_connected_rooms(&self) -> bool {
        matches!(self, BuilderState::PlacingPlayer | BuilderState::Finished)
    }

    pub fn has_placed_player(&self) -> bool {
        matches!(self, BuilderState::Finished)
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, BuilderState::Finished)
    }
}

impl Default for BuilderState {
    fn default() -> Self {
        Self::Started
    }
}

pub struct MapBuilder {
    pub map: Map,
    pub num_of_rooms: usize,
    pub rooms: Vec<Rect>,
    pub tunnels: Vec<Tunnel>,
    pub player: Player,
    pub state: BuilderState,
}

impl MapBuilder {
    pub fn new(width: i32, height: i32, number_of_rooms: usize) -> Self {
        Self {
            map: Map::new(width, height),
            rooms: Vec::with_capacity(number_of_rooms),
            tunnels: Vec::with_capacity(number_of_rooms * 2),
            player: Player::new(0, 0),
            state: BuilderState::default(),
            num_of_rooms: number_of_rooms,
        }
    }

    pub fn build(self) -> MapBuilderResult {
        MapBuilderResult {
            map: self.map,
            player: self.player,
        }
    }

    pub fn next(&mut self, rng: &mut RandomNumberGenerator) {
        self.state.next();
        match self.state {
            BuilderState::Filling => self.fill(),
            BuilderState::Rooms => self.build_rooms(rng),
            BuilderState::ConnectingRooms => self.build_tunnels(rng),
            BuilderState::PlacingPlayer => self.place_player(rng),
            BuilderState::Finished => self.build_map(),
            _ => {}
        }
    }

    pub fn fill(&mut self) {
        self.map.fill(TileType::Wall);
    }

    fn place_player(&mut self, rng: &mut RandomNumberGenerator) {
        let room = rng.range(0, self.num_of_rooms);
        let room = self.rooms[room].center();
        self.player.position = room;
    }

    fn build_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < self.num_of_rooms {
            let max_room_size = 10;
            let room = Rect::with_size(
                rng.range(1, self.map.width - max_room_size),
                rng.range(1, self.map.height - max_room_size),
                rng.range(2, max_room_size),
                rng.range(2, max_room_size),
            );

            let mut overlap = false;
            // This can be skipped forward
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if overlap {
                continue;
            }
            self.rooms.push(room);
        }
    }

    fn build_map(&mut self) {
        for room in self.rooms.iter() {
            self.map.carve_room(room, TileType::Floor);
        }
        for tunnel in self.tunnels.iter() {
            self.map.carve_tunnel(tunnel, TileType::Floor);
        }
    }

    fn build_tunnels(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.tunnels.push(Tunnel::horizontal(prev.x, new.x, prev.y));
                self.tunnels.push(Tunnel::vertical(prev.y, new.y, new.x));
            } else {
                self.tunnels.push(Tunnel::vertical(prev.y, new.y, prev.x));
                self.tunnels.push(Tunnel::horizontal(prev.x, new.x, new.y));
            }
        }
    }

    pub fn render(&self, draw: &mut DrawBatch) {
        self.map.render(draw);
        if let BuilderState::Rooms = self.state {
            self.rooms.iter().for_each(|room| {
                draw.fill_region(*room, ColorPair::new(RED, BLACK), TileType::Floor);
            });
        };
        if let BuilderState::ConnectingRooms | BuilderState::PlacingPlayer = self.state {
            for room in self.rooms.iter() {
                draw.fill_region(*room, ColorPair::new(RED, BLACK), TileType::Floor);
            }
            for tunnel in self.tunnels.iter() {
                tunnel.render(draw);
            }
        }
        if let BuilderState::Finished | BuilderState::PlacingPlayer = self.state {
            self.player.render(draw);
        }
    }

    pub fn draw_menu(&self, draw: &mut DrawBatch) {
        let margin = 5;
        let menu_width = 20;
        let x = SCREEN_WIDTH - (menu_width + margin);
        let h = 11;
        let modal = Rect::with_size(x, margin, menu_width, h);
        draw.draw_double_box(modal, ColorPair::new(GREY, BLACK));

        let mut buf = TextBuilder::empty();
        let mut block = TextBlock::new(x + 1, margin + 1, menu_width - 1, h - 1);

        let fill_map_text = if self.state.has_filled() {
            "[X] Fill Map"
        } else {
            "[ ] Fill Map"
        };
        let generate_room_text = if self.state.has_build_rooms() {
            "[X] Generate Rooms"
        } else {
            "[ ] Generate Rooms"
        };
        let coonnect_rooms_text = if self.state.has_connected_rooms() {
            "[X] Connect Rooms"
        } else {
            "[ ] Connect Rooms"
        };

        let place_player_text = if self.state.has_placed_player() {
            "[X] Place Player"
        } else {
            "[ ] Place Player"
        };
        let finished_text = if self.state.is_finished() {
            "[X] Finished"
        } else {
            "[ ] Finished"
        };

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
            BuilderState::Filling => {
                draw.set(
                    Point::new(x + 2, margin + 4),
                    ColorPair::new(BLACK, YELLOW),
                    to_cp437('>'),
                );
            }
            BuilderState::Rooms => {
                draw.set(
                    Point::new(x + 2, margin + 5),
                    ColorPair::new(BLACK, YELLOW),
                    to_cp437('>'),
                );
            }
            BuilderState::ConnectingRooms => {
                draw.set(
                    Point::new(x + 2, margin + 6),
                    ColorPair::new(BLACK, YELLOW),
                    to_cp437('>'),
                );
            }
            BuilderState::PlacingPlayer => {
                draw.set(
                    Point::new(x + 2, margin + 7),
                    ColorPair::new(BLACK, YELLOW),
                    to_cp437('>'),
                );
            }
            _ => {}
        };
    }
}

pub struct MapBuilderResult {
    pub map: Map,
    pub player: Player,
}

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
}

impl From<TileType> for FontCharType {
    fn from(val: TileType) -> Self {
        match val {
            TileType::Floor => to_cp437(','),
            TileType::Wall => to_cp437('#'),
        }
    }
}