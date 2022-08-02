use graphics::{Context, Rectangle};
use opengl_graphics::{GlGraphics, GlyphCache};
use serde::{Serialize, Deserialize};

use super::RenderJob;
#[derive(Clone, Serialize, Deserialize)]
pub struct Rect {
    pub color: [f32; 4],
    pub bounds: [f64; 4],
}
impl Rect {
    pub fn render(&self, context: &Context, graphics: &mut GlGraphics) {
        Rectangle::new(self.color).draw(self.bounds, &context.draw_state, context.transform, graphics);
    }
    pub const fn new(color: [f32; 4], bounds: [f64; 4]) -> RenderJob {
        RenderJob::Rect(Rect {color, bounds})
    }
    pub fn ensure_mut(orig: &mut RenderJob) -> &mut Rect {
        match orig {
            RenderJob::Rect(res) => res,
            _ => panic!("Ensure failed!")
        }
    }
    pub fn ensure(orig: &RenderJob) -> &Rect {
        match orig {
            RenderJob::Rect(res) => res,
            _ => panic!("Ensure failed!")
        }
    }
}