use graphics::Context;
use opengl_graphics::{GlGraphics, GlyphCache};
use serde::{Serialize, Deserialize};

use super::{RenderJobComponent, RenderJob, texture::TextureBuffer};

#[derive(Clone, Serialize, Deserialize)]
pub struct Toggle {
    pub jobs: Vec<RenderJobComponent>,
    pub enabled: usize, // which job is enabled
    pub bounds: [f64; 4],
    pub tint: [f32; 4],
    pub anim: bool, // if true, enabled increases by 1 each frames
}
impl Toggle {
    pub fn new(possibilities: Vec<RenderJobComponent>, enabled: usize, bounds: [f64; 4], tint: [f32; 4], animation: bool) -> RenderJob {
        RenderJob {enabled: true, 
            cmp: RenderJobComponent::Toggle(
                Toggle {
                    jobs: possibilities,
                    enabled,
                    bounds,
                    tint,
                    anim: animation,
                }
            )
        }
    }
    pub fn render(&mut self, context: &Context, graphics: &mut GlGraphics, font: &mut Vec<GlyphCache>, textures: &TextureBuffer) {
        let mut job2 = self.jobs[self.enabled].clone();
        let bounds = job2.bounds();
        bounds[0] = self.bounds[0] + bounds[0] * self.bounds[2];
        bounds[1] = self.bounds[1] + bounds[1] * self.bounds[3];
        bounds[2] *= self.bounds[2];
        bounds[3] *= self.bounds[3];
        let tint = job2.tint();
        tint[0] *= self.tint[0];
        tint[1] *= self.tint[1];
        tint[2] *= self.tint[2];
        tint[3] *= self.tint[3];
        job2.render(context, graphics, font, textures);
        if self.anim {
            self.enabled = (self.enabled + 1) % self.jobs.len();
        }
    }
}