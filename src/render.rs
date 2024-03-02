pub mod composite;
pub mod rect;
pub mod text;
pub mod texture;
pub mod toggle;

use std::{collections::BTreeMap, thread::sleep, time::Instant};

use graphics::{color::BLACK, Context, Graphics};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{
    Button, ButtonState, Event, EventSettings, Events, Input, Loop, Motion, RenderArgs, UpdateArgs,
    WindowSettings,
};
use piston_window::PistonWindow;
use serde::{Deserialize, Serialize};

use crate::{
    consts::{FRAMERATE, LAYERS, LAYER_SIZE, OPENGL, TITLE, TRANSPARENT},
    input::InputVars,
    render::text::TextRenderer,
};

use self::{
    composite::Composite,
    rect::Rect,
    texture::{ImageRenderer, TextureBuffer},
    toggle::Toggle,
};

pub struct Window<'a> {
    pub window: PistonWindow,
    pub gl: GlGraphics,
    pub events: Events,
    pub jobs: RenderJobs,
    pub fonts: Vec<GlyphCache<'a>>,
    pub last_time: Instant,
    pub input: InputVars,
    pub textures: TextureBuffer,
}
impl<'a> Window<'a> {
    pub fn new(fonts: Vec<GlyphCache<'a>>, x: u32, y: u32) -> Window<'a> {
        let window: PistonWindow = WindowSettings::new(TITLE, [x, y])
            .graphics_api(OPENGL)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let gl = GlGraphics::new(OPENGL);
        let events = Events::new(EventSettings::new());
        let jobs = RenderJobs::new();
        Window {
            window,
            gl,
            events,
            jobs,
            fonts,
            last_time: Instant::now(),
            input: InputVars::new(),
            textures: TextureBuffer::new(),
        }
    }
    pub fn render(&mut self, args: &RenderArgs) {
        let job_list: Vec<&mut RenderJob> = self.jobs.all_jobs().collect();
        let gl = &mut self.gl;
        let fonts = &mut self.fonts;
        let textures = &self.textures;
        gl.draw(args.viewport(), |c, g| {
            g.clear_color(BLACK);
            for job in job_list {
                job.render(&c, g, fonts, textures)
            }
        });
    }
    pub fn update(&mut self, _args: &UpdateArgs) {}
    pub fn handle_input(&mut self, input: Input) -> bool {
        match input {
            Input::Button(val) => match val.button {
                Button::Keyboard(key) => {
                    self.input
                        .update_key(key as u32, val.state == ButtonState::Press);
                }
                Button::Mouse(mouse) => {
                    self.input.update_mouse(
                        1 << (mouse as u8).saturating_sub(1),
                        val.state == ButtonState::Press,
                    );
                }
                Button::Controller(_) => todo!(),
                Button::Hat(_) => todo!(),
            },
            Input::Move(val) => {
                match val {
                    Motion::MouseCursor(pos) => {
                        self.input.mouse_pos = pos;
                    }
                    Motion::MouseScroll(_) => {
                        // TODO: Insert scroll behavior when needed
                    }
                    _ => {}
                }
            }
            Input::Text(text) => {
                self.input.text_buffer.push_str(&text);
            }
            Input::Resize(val) => {
                self.input.update_dimension(val.window_size);
            }
            Input::Focus(val) => {
                self.input.focus(val);
            }
            Input::Cursor(val) => {
                self.input.cursor(val);
            }
            Input::FileDrag(_) => {}
            Input::Close(_) => return false,
        };
        true
    }
    // runs a single frame. Returns false if the loop should end.
    pub fn run_loop_iteration(&mut self) -> bool {
        self.input.reset();
        let now = Instant::now();
        let to_sleep = FRAMERATE.saturating_sub(now - self.last_time);
        sleep(to_sleep);
        self.last_time = Instant::now();
        while let Some(e) = self.events.next(&mut self.window) {
            match e {
                Event::Input(input, _) => {
                    if !self.handle_input(input) {
                        return false;
                    }
                }
                Event::Loop(val) => match val {
                    Loop::Render(args) => self.render(&args),
                    Loop::AfterRender(_) => return true,
                    Loop::Update(args) => self.update(&args),
                    Loop::Idle(_) => (),
                },
                Event::Custom(_, _, _) => (),
            }
        }
        false
    }
}
// A structure that handles items to render.
pub struct RenderJobs {
    internal: BTreeMap<RenderJobID, RenderJob>,
    count: Vec<u64>, // one count is kept per layer
}
impl RenderJobs {
    pub fn new() -> RenderJobs {
        RenderJobs {
            internal: BTreeMap::new(),
            count: vec![0; LAYERS as usize],
        }
    }
    pub fn add_job(&mut self, job: RenderJob, layer: u64) -> RenderJobID {
        let res = RenderJobID(self.count[layer as usize] + layer * LAYER_SIZE);
        self.internal.insert(res, job);
        self.count[layer as usize] += 1;
        res
    }
    pub fn set_job(&mut self, job: RenderJob, id: RenderJobID) {
        self.internal.insert(id, job);
    }
    pub fn get_job_mut(&mut self, id: RenderJobID) -> Option<&mut RenderJob> {
        self.internal.get_mut(&id)
    }
    pub fn get_job(&self, id: RenderJobID) -> Option<&RenderJob> {
        self.internal.get(&id)
    }
    pub fn remove_job(&mut self, id: RenderJobID) -> Option<RenderJob> {
        self.internal.remove(&id)
    }
    pub fn all_jobs(&mut self) -> impl Iterator<Item = &mut RenderJob> {
        self.internal.values_mut()
    }
    pub fn get_layer(id: RenderJobID) -> u64 {
        id.0 / LAYER_SIZE
    }
}
// a render job ID.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderJobID(u64);
// the type of rendering to be done - a square, a circle, or even an image
#[derive(Clone, Serialize, Deserialize)]
pub enum RenderJobComponent {
    Rect(Rect),
    Text(TextRenderer),
    Image(ImageRenderer),
    Composite(Composite),
    Toggle(Toggle),
}
impl RenderJobComponent {
    pub fn render(
        &mut self,
        context: &Context,
        graphics: &mut GlGraphics,
        font: &mut Vec<GlyphCache>,
        textures: &TextureBuffer,
    ) {
        match self {
            RenderJobComponent::Rect(val) => val.render(context, graphics),
            RenderJobComponent::Text(val) => val.render(context, graphics, font),
            RenderJobComponent::Image(val) => val.render(context, graphics, textures),
            RenderJobComponent::Composite(val) => val.render(context, graphics, font, textures),
            RenderJobComponent::Toggle(val) => val.render(context, graphics, font, textures),
        }
    }
    pub fn bounds(&mut self) -> &mut [f64; 4] {
        match self {
            RenderJobComponent::Rect(val) => &mut val.bounds,
            RenderJobComponent::Text(val) => &mut val.bounds,
            RenderJobComponent::Image(val) => &mut val.bounds,
            RenderJobComponent::Composite(val) => val.bounds(),
            RenderJobComponent::Toggle(val) => &mut val.bounds,
        }
    }
    pub fn tint(&mut self) -> &mut [f32; 4] {
        match self {
            RenderJobComponent::Rect(val) => &mut val.color,
            RenderJobComponent::Text(val) => &mut val.color,
            RenderJobComponent::Image(val) => &mut val.tint,
            RenderJobComponent::Composite(val) => val.tint(),
            RenderJobComponent::Toggle(val) => &mut val.tint,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct RenderJob {
    pub cmp: RenderJobComponent,
    pub enabled: bool,
}
impl RenderJob {
    pub fn default() -> RenderJob {
        Rect::new(TRANSPARENT, [0.0; 4])
    }
    fn render(
        &mut self,
        context: &Context,
        graphics: &mut GlGraphics,
        font: &mut Vec<GlyphCache>,
        textures: &TextureBuffer,
    ) {
        if self.enabled {
            self.cmp.render(context, graphics, font, textures);
        }
    }
    pub fn bounds(&mut self) -> &mut [f64; 4] {
        self.cmp.bounds()
    }
}
