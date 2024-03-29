use graphics::{Context, Image};
use opengl_graphics::{GlGraphics, Texture};
use serde::{Deserialize, Serialize};

use super::{RenderJob, RenderJobComponent};

pub struct TextureBuffer {
    pub textures: Vec<Texture>,
}
impl TextureBuffer {
    pub fn new() -> TextureBuffer {
        TextureBuffer {
            textures: Vec::new(),
        }
    }
    pub fn add(&mut self, texture: Texture) -> TextureID {
        self.textures.push(texture);
        TextureID(self.textures.len() - 1)
    }
    pub fn get(&self, id: &TextureID) -> &Texture {
        &self.textures[id.0]
    }
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TextureID(pub usize);
#[derive(Clone, Serialize, Deserialize)]
pub struct ImageRenderer {
    pub bounds: [f64; 4],
    pub tint: [f32; 4],
    pub texture: TextureID,
}
impl ImageRenderer {
    pub fn render(&self, context: &Context, graphics: &mut GlGraphics, textures: &TextureBuffer) {
        Image::new().color(self.tint).rect(self.bounds).draw(
            textures.get(&self.texture),
            &context.draw_state,
            context.transform,
            graphics,
        );
    }
    pub const fn new(bounds: [f64; 4], tint: [f32; 4], texture: TextureID) -> RenderJob {
        RenderJob {
            cmp: RenderJobComponent::Image(ImageRenderer {
                bounds,
                tint,
                texture,
            }),
            enabled: true,
        }
    }
    
    // Attempts to convert a renderjob into a text object. Panics if it fails. 
    pub fn ensure_mut(orig: &mut RenderJob) -> &mut ImageRenderer {
        match &mut orig.cmp {
            RenderJobComponent::Image(res) => res,
            _ => panic!("Ensure failed!"),
        }
    }
    // Attempts to convert a renderjob into a text object. Panics if it fails. 
    pub fn ensure(orig: &RenderJob) -> &ImageRenderer {
        match &orig.cmp {
            RenderJobComponent::Image(res) => res,
            _ => panic!("Ensure failed!"),
        }
    }
}
