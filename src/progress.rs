use crate::prelude::*;

pub trait RenderProgress {
    fn render_progress(&self, draw: &mut DrawBatch);
}
