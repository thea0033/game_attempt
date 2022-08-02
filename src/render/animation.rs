use graphics::Context;
use opengl_graphics::{GlGraphics, GlyphCache};

use super::{RenderJob, texture::TextureBuffer};

pub struct Animation {
    current: usize,
    contents: Vec<RenderJob>,
}
impl Animation {
    pub fn render(&mut self, context: &Context, graphics: &mut GlGraphics, font: &mut Vec<GlyphCache>, textures: &TextureBuffer ) {
        self.contents[self.current].render(context, graphics, font, textures);
        self.current += 1;
        if self.current >= self.contents.len() {
            self.current = 0;
        }
    }
    pub fn new(contents: Vec<RenderJob>) -> RenderJob {
        todo!()
    }
}