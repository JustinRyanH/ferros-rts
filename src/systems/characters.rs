use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.update(want_move.destination);
        }
    }
    commands.remove(*entity);
}

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // TODO: This should be a resource, so we can make RNG more predictable
    let mut rng = RandomNumberGenerator::new();
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut attacked = false;

    let movers: Vec<(Entity, Point)> = movers
        .iter_mut(ecs)
        .map(|(entity, pos, _)| {
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;
            (*entity, destination)
        })
        .collect();

    for (entity, destination) in movers {
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(victim, _, _)| {
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: entity,
                            victim: *victim,
                        },
                    ));
                    attacked = true;
                }
            });

        if !attacked {
            commands.push((
                (),
                WantsToMove {
                    entity,
                    destination,
                },
            ));
        }
    }
}

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attacks = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity)> = attacks
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();
    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}
