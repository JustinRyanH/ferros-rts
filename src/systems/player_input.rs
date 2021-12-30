use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let key = match key {
        Some(it) => it,
        _ => return,
    };
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let delta = match key {
        VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
        VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
        VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
        VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
        _ => Point::new(0, 0),
    };

    let (player_entity, destination) = players
        .iter(ecs)
        .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
        .unwrap();

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    let mut did_something = false;
    if delta.x != 0 || delta.y != 0 {
        let mut hit_something = false;
        did_something = true;
        enemies
            .iter(ecs)
            .filter(|(_, pos)| **pos == destination)
            .for_each(|(entity, _)| {
                hit_something = true;
                commands.push((
                    (),
                    WantsToAttack {
                        attacker: player_entity,
                        victim: *entity,
                    },
                ));
            });
        if !hit_something {
            commands.push((
                (),
                WantsToMove {
                    entity: player_entity,
                    destination,
                },
            ));
        }
    }

    if !did_something {
        if let Ok(mut health) = ecs
            .entry_mut(player_entity)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current = (health.current + 1).min(health.max);
        }
    }

    *turn_state = TurnState::PlayerTurn;
}
