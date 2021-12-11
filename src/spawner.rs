use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let render = Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@'),
    };
    ecs.push((Player, pos, render));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) -> Entity {
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(RED, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
    ))
}
