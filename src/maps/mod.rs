mod map;

use crate::prelude::*;
pub use map::*;

pub struct MapResult {
    pub map: Map,
    pub player: Option<Point>,
}

#[derive(Debug, Clone, Copy)]
pub enum BuildCommandResult {
    Finished,
    Progress { total: usize, current: usize },
}

pub struct MapBuilder {
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Rect>,
    pub tunnels: Vec<Tunnel>,
    pub player: Option<Point>,
    pub fill_tile: Option<TileType>,
    pub finished: bool,
    pub point: Point,
}

impl MapBuilder {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            rooms: Vec::with_capacity(100),
            tunnels: Vec::with_capacity(200),
            player: None,
            fill_tile: None,
            finished: false,
            point: Point::zero(),
        }
    }

    pub fn fill(&mut self, tile: &TileType) -> BuildCommandResult {
        self.fill_tile = Some(*tile);
        BuildCommandResult::Finished
    }

    pub fn place_player(&mut self, rng: &mut RandomNumberGenerator) -> BuildCommandResult {
        let room = rng.range(0, self.rooms.len());
        let room = self.rooms[room].center();
        self.point = room;
        self.player = Some(room);
        BuildCommandResult::Finished
    }

    pub fn build_room(
        &mut self,
        num_of_rooms: i32,
        max_room_size: i32,
        rng: &mut RandomNumberGenerator,
    ) -> BuildCommandResult {
        let num_of_rooms = num_of_rooms as usize;
        if self.rooms.len() >= num_of_rooms {
            self.rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
            return BuildCommandResult::Finished;
        }

        let room = Rect::with_size(
            rng.range(1, self.width - max_room_size),
            rng.range(1, self.height - max_room_size),
            rng.range(2, max_room_size),
            rng.range(2, max_room_size),
        );

        if !self.rooms.iter().any(|r| r.intersect(&room)) {
            self.rooms.push(room);
            self.point = room.center();
        }
        BuildCommandResult::Progress {
            total: num_of_rooms,
            current: self.rooms.len(),
        }
    }

    pub fn build_tunnels(
        &mut self,
        max_tunnels: i32,
        rng: &mut RandomNumberGenerator,
    ) -> BuildCommandResult {
        let max_tunnels = max_tunnels as usize;
        if self.tunnels.len() >= max_tunnels {
            return BuildCommandResult::Finished;
        }

        let connecting_room_index = (self.tunnels.len() / 2) + 1;
        let prev = self.rooms[connecting_room_index - 1].center();
        let new = self.rooms[connecting_room_index].center();

        self.point = (prev + new) / 2;

        if rng.range(0, 2) == 1 {
            self.tunnels.push(Tunnel::horizontal(prev.x, new.x, prev.y));
            self.tunnels.push(Tunnel::vertical(prev.y, new.y, new.x));
        } else {
            self.tunnels.push(Tunnel::vertical(prev.y, new.y, prev.x));
            self.tunnels.push(Tunnel::horizontal(prev.x, new.x, new.y));
        }

        BuildCommandResult::Progress {
            total: max_tunnels,
            current: self.tunnels.len(),
        }
    }

    pub fn build_map(&mut self) -> MapResult {
        let mut map = Map::new(self.width, self.height);
        for tile in self.fill_tile.iter() {
            map.fill(*tile);
        }
        for room in self.rooms.iter() {
            map.carve_room(room, TileType::Floor);
        }
        for tunnel in self.tunnels.iter() {
            map.carve_tunnel(tunnel, TileType::Floor);
        }
        MapResult {
            map,
            player: self.player,
        }
    }
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

impl From<TileType> for ColorPair {
    fn from(val: TileType) -> Self {
        let fg_light = RGBA::from_f32(1.0, 1.0, 0.0, 0.5);
        let fg_mid = RGBA::from_f32(1.0, 1.0, 0.0, 0.75);
        match val {
            TileType::Floor => ColorPair::new(fg_light, BLACK),
            TileType::Wall => ColorPair::new(fg_mid, BLACK),
        }
    }
}
