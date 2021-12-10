use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let render = Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@'),
    };
    ecs.push((Player, pos, render));
}
