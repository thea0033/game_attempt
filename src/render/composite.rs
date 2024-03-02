use graphics::Context;
use opengl_graphics::{GlGraphics, GlyphCache};
use serde::{Deserialize, Serialize};

use super::{texture::TextureBuffer, RenderJob, RenderJobComponent};

#[derive(Clone, Serialize, Deserialize)]
pub struct Composite {
    jobs: Vec<RenderJobComponent>,
    cache: Vec<Option<RenderJobComponent>>,
    enabled: Vec<bool>,
    bounds: [f64; 4],
    tint: [f32; 4],
}
impl Composite {
    pub fn bounds(&mut self) -> &mut [f64; 4] {
        self.cache
            .iter_mut()
            .map(|x| *x = None)
            .for_each(std::mem::drop);
        &mut self.bounds
    }
    pub fn tint(&mut self) -> &mut [f32; 4] {
        self.cache
            .iter_mut()
            .map(|x| *x = None)
            .for_each(std::mem::drop);
        &mut self.tint
    }
    // Attempts to convert a renderjob into a composite. Panics if it fails. 
    pub fn ensure_mut(orig: &mut RenderJob) -> &mut Composite {
        match &mut orig.cmp {
            RenderJobComponent::Composite(res) => res,
            _ => panic!("Ensure failed!"),
        }
    }
    // Attempts to convert a renderjob into a composite. Panics if it fails. 
    pub fn ensure(orig: &RenderJob) -> &Composite {
        match &orig.cmp {
            RenderJobComponent::Composite(res) => res,
            _ => panic!("Ensure failed!"),
        }
    }
    pub fn new(bounds: [f64; 4], tint: [f32; 4]) -> RenderJob {
        RenderJob {
            enabled: true,
            cmp: RenderJobComponent::Composite(Composite {
                jobs: Vec::new(),
                enabled: Vec::new(),
                cache: Vec::new(),
                bounds,
                tint,
            }),
        }
    }
    pub fn add_job(&mut self, job: RenderJob, enabled: bool) {
        self.jobs.push(job.cmp.clone());
        self.enabled.push(enabled);
        self.cache.push(None);
    }
    pub fn update_cache(&mut self, index: usize) {
        let mut job2 = self.jobs[index].clone();
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
        self.cache[index] = Some(job2);
    }
    pub fn toggle_job(&mut self, index: usize, state: bool) {
        self.enabled[index] = state;
    }
    pub fn get_job(&mut self, index: usize) -> &mut RenderJobComponent {
        self.cache[index] = None;
        &mut self.jobs[index]
    }
    pub fn render(
        &mut self,
        context: &Context,
        graphics: &mut GlGraphics,
        font: &mut Vec<GlyphCache>,
        textures: &TextureBuffer,
    ) {
        for i in 0..self.cache.len() {
            if self.enabled[i] {
                if let None = self.cache[i] {
                    self.update_cache(i);
                }
                let job = self.cache[i].as_mut().unwrap();
                job.render(context, graphics, font, textures);
            }
        }
    }
}
