use graphics::{Context, Rectangle};
use opengl_graphics::{GlGraphics, GlyphCache};
use serde::{Serialize, Deserialize};

use super::{RenderJob, RenderJobComponent};
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
        RenderJob {
            cmp: RenderJobComponent::Rect(Rect {color, bounds}),
            enabled: true
        }
    }
    pub fn ensure_mut(orig: &mut RenderJob) -> &mut Rect {
        match &mut orig.cmp {
            RenderJobComponent::Rect(res) => res,
            _ => panic!("Ensure failed!")
        }
    }
    pub fn ensure(orig: &RenderJob) -> &Rect {
        match &orig.cmp {
            RenderJobComponent::Rect(res) => res,
            _ => panic!("Ensure failed!")
        }
    }
}