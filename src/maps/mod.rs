mod map;

pub use map::*;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub tunnels: Vec<Tunnel>,
    pub player: Option<Player>,
}

impl MapBuilder {
    pub fn new(width: i32, height: i32, number_of_rooms: usize) -> Self {
        Self {
            map: Map::new(width, height),
            rooms: Vec::with_capacity(number_of_rooms),
            tunnels: Vec::with_capacity(number_of_rooms * 2),
            player: None,
        }
    }

    pub fn place_player(&mut self, rng: &mut RandomNumberGenerator) {
        let room = rng.range(0, self.rooms.len());
        let room = self.rooms[room].center();
        self.player = Some(Player::new(room.x, room.y));
    }

    pub fn build_rooms(
        &mut self,
        num_of_rooms: i32,
        max_room_size: i32,
        rng: &mut RandomNumberGenerator,
    ) {
        while self.rooms.len() < num_of_rooms as usize {
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

    pub fn build_map(&mut self) {
        for room in self.rooms.iter() {
            self.map.carve_room(room, TileType::Floor);
        }
        for tunnel in self.tunnels.iter() {
            self.map.carve_tunnel(tunnel, TileType::Floor);
        }
    }

    pub fn build_tunnels(&mut self, rng: &mut RandomNumberGenerator) {
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
        for room in self.rooms.iter() {
            draw.fill_region(*room, ColorPair::new(RED, BLACK), TileType::Floor);
        }
        for tunnel in self.tunnels.iter() {
            tunnel.render(draw);
        }
        for player in self.player.iter() {
            player.render(draw)
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
