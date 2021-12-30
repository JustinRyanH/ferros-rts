use crate::prelude::*;

fn goblin() -> (Health, Name, FontCharType) {
    (Health::new(1), "Goblin".into(), to_cp437('g'))
}

fn orc() -> (Health, Name, FontCharType) {
    (Health::new(2), "Orc".into(), to_cp437('o'))
}

pub fn spawn_player(commands: &mut CommandBuffer, pos: Point) {
    let render = Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@'),
    };
    commands.push((
        Player,
        pos,
        render,
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_monster(
    commands: &mut CommandBuffer,
    rng: &mut RandomNumberGenerator,
    pos: Point,
) -> Entity {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=9 => goblin(),
        _ => orc(),
    };

    let color = ColorPair::new(RED, BLACK);
    commands.push((
        Enemy,
        pos,
        Render { color, glyph },
        MovingRandomly {},
        hp,
        name,
    ))
}
