use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        players.iter(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
        *turn_state = TurnState::PlayerTurn;
    }
}
