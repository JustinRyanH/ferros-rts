use crate::prelude::*;

pub struct PlayerState {
    pub player: Player,
    pub map: Map,
}

impl PlayerState {
    pub fn new() -> Self {
        let MapBuilderResult { player, map } =
            MapBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, 1).build();
        Self { player, map }
    }

    fn render_tick(&mut self, ctx: &mut BTerm) {
        let mut draw = DrawBatch::new();
        Self::clear_targets(&mut draw);

        self.player.render(&mut draw);
        self.map.render(&mut draw);

        draw.submit(0).expect("Batch Error");
        render_draw_buffer(ctx).expect("Render Error");
    }

    fn clear_targets(draw: &mut DrawBatch) {
        draw.target(0);
        draw.cls();
        draw.target(1);
        draw.cls();
    }
}

impl GameState for PlayerState {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        self.player.update(ctx);
        self.render_tick(ctx);
    }
}
