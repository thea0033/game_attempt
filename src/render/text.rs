use std::borrow::Cow;

use graphics::{types::FontSize, CharacterCache, Context, Text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::Size;
use serde::{Deserialize, Serialize};

use super::{RenderJob, RenderJobComponent};

#[derive(Clone, Serialize, Deserialize)]
pub struct FontID(pub usize);
#[derive(Clone, Serialize, Deserialize)]
pub struct TextRenderer {
    pub text: Cow<'static, str>,
    pub bounds: [f64; 4],
    pub color: [f32; 4],
    pub size: FontSize,
    pub halign: i8, // -1 = left, 0 = center, 1 = right
    pub valign: i8, // -1 = top, 0 = center, 1 = bottom
    pub font: FontID,
}
impl TextRenderer {
    pub const fn new(
        text: String,
        bounds: [f64; 4],
        color: [f32; 4],
        size: FontSize,
        halign: i8,
        valign: i8,
        font: FontID,
    ) -> RenderJob {
        RenderJob {
            cmp: RenderJobComponent::Text(TextRenderer {
                text: Cow::Owned(text),
                bounds,
                color,
                size,
                halign,
                valign,
                font,
            }),
            enabled: true,
        }
    }
    pub const fn new_ref(
        text: &'static str,
        bounds: [f64; 4],
        color: [f32; 4],
        size: FontSize,
        halign: i8,
        valign: i8,
        font: FontID,
    ) -> RenderJob {
        RenderJob {
            cmp: RenderJobComponent::Text(TextRenderer {
                text: Cow::Borrowed(text),
                bounds,
                color,
                size,
                halign,
                valign,
                font,
            }),
            enabled: true,
        }
    }
    
    // Attempts to convert a renderjob into a text object. Panics if it fails. 
    pub fn ensure_mut(orig: &mut RenderJob) -> &mut TextRenderer {
        match &mut orig.cmp {
            RenderJobComponent::Text(res) => res,
            _ => panic!("Ensure failed!"),
        }
    }
    // Attempts to convert a renderjob into a text object. Panics if it fails. 
    pub fn ensure(orig: &RenderJob) -> &TextRenderer {
        match &orig.cmp {
            RenderJobComponent::Text(res) => res,
            _ => panic!("Ensure failed!"),
        }
    }
    pub fn render(&self, context: &Context, graphics: &mut GlGraphics, font: &mut Vec<GlyphCache>) {
        let x0 = self.bounds[0];
        let y0 = self.bounds[1];
        let x1 = self.bounds[2];
        let y1 = self.bounds[3];

        let t = Text::new_color(self.color, self.size);
        let size = measure(&self.text, &mut font[self.font.0], self.size);
        fn center(p0: f64, p1: f64, wh: f64) -> f64 {
            p0 + ((p1 - p0) / 2.0) - (wh / 2.0)
        }
        let x = match self.halign {
            -1 => x0,
            1 => x1 - size.width,
            _ => center(x0, x1, size.width),
        };

        let y = match self.valign {
            -1 => y0,
            1 => y1 - size.height,
            _ => center(y0, y1, size.height),
        };

        let transform = context.transform.trans(x, y);
        let draw_state = context.draw_state;
        t.draw(
            &self.text,
            &mut font[self.font.0],
            &draw_state,
            transform,
            graphics,
        )
        .unwrap();
    }
}
fn measure(text: &str, cache: &mut GlyphCache, fs: FontSize) -> Size {
    let mut w = 0.0;
    let mut h = 0.0;
    for ch in text.chars() {
        let character = cache.character(fs, ch).ok().unwrap();
        let (left, top) = (character.left(), character.top());
        w += character.advance_width() + left;
        h = (character.advance_height() + top).max(h);
    }
    Size {
        width: w as f64,
        height: h as f64,
    }
}
