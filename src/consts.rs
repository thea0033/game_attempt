pub mod objects;
pub mod colors;
pub mod layers;

use std::time::Duration;

use opengl_graphics::OpenGL;

pub use self::objects::*;
pub use self::colors::*;
pub use self::layers::*;

use crate::{render::{text::FontID}};
pub const ASSETS_FOLDER: &str = "assets";
// text rendering
pub const DEFAULT_FONT_PATH: &str = "InconsolataZi4varlvarquRegular-42gD.ttf";
pub const DEFAULT_FONT_ID: FontID = FontID(0);
// window stuff
pub const TITLE:&str = "The Horrible Game";
// each tile is 50 pixels
pub const GRID_SIZE: f64 = 50.0;
// the grid is 20 tiles by 20 tiles
pub const WINDOW_X:u32 = (GRID_SIZE * 20.0) as u32;
pub const WINDOW_Y:u32 = (GRID_SIZE * 20.0) as u32;
pub const OPENGL: OpenGL = OpenGL::V4_5;
pub const FRAMERATE: Duration = Duration::from_micros(0); // no maximum framerate
// input
pub const LEFT_MOUSE: u8 = 1;
pub const RIGHT_MOUSE: u8 = 2;
pub const MIDDLE_MOUSE: u8 = 4;
pub const ANY_MOUSE: u8 = 255;
// the fugde factor. Allows any magnitudes less than this (outside of comparisons to 0) to pass equality and gt/lt checks. 
pub const FUDGE: f64 = 1.0;
// how many times collision is checked. 
pub const NUM_TIMES: u32 = 8;
pub const NUM_TIMES_F64: f64 = NUM_TIMES as f64;