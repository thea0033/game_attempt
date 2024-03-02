pub mod colors;
pub mod layers;
pub mod objects;

use std::time::Duration;

use opengl_graphics::OpenGL;

pub use self::colors::*;
pub use self::layers::*;
pub use self::objects::*;

use crate::render::text::FontID;
pub const ASSETS_FOLDER: &str = "assets";
// text rendering
pub const DEFAULT_FONT_PATH: &str = "fonts\\InconsolataZi4varlvarquRegular-42gD.ttf";
pub const DEFAULT_FONT_ID: FontID = FontID(0);
// window stuff
pub const TITLE: &str = "The Horrible Game";
// each tile is 50 pixels
pub const GRID_SIZE: f64 = 15.0;
pub const TILES: usize = 50;
pub const TILES_U32: u32 = 50;
// the grid is 20 tiles by 20 tiles
pub const WINDOW_X: u32 = (GRID_SIZE as u32) * TILES_U32;
pub const WINDOW_Y: u32 = (GRID_SIZE as u32) * TILES_U32;
// Map editor dimensions: 52 by 52 board, smaller tiles, and 12 tiles worth of extra room on the right. 
pub const MEDIT_TILES: u32 = 52;
pub const MEDIT_TILE_SIZE: f64 = 12.5;
pub const MEDIT_EXTRA_ROOM: u32 = 12; // 12 tiles worth of extra room
pub const MEDIT_GUIDE_SIZE: u32 = 3;
pub const MEDIT_WINDOW_X: u32 = (MEDIT_TILE_SIZE as u32) * (MEDIT_TILES + MEDIT_EXTRA_ROOM);
pub const MEDIT_WINDOW_Y: u32 = (MEDIT_TILE_SIZE as u32) * MEDIT_TILES;
// Opengl stuff
pub const OPENGL: OpenGL = OpenGL::V4_5;
pub const FRAMERATE: Duration = Duration::from_micros(0); // no maximum framerate

// input
pub const LEFT_MOUSE: u8 = 1;
pub const RIGHT_MOUSE: u8 = 2;
pub const MIDDLE_MOUSE: u8 = 4;
pub const ANY_MOUSE: u8 = 255;

// the fugde factor. Allows any magnitudes less than this (outside of comparisons to 0) to pass equality and gt/lt checks.
pub const FUDGE: f64 = 0.01 * GRID_SIZE;

// how many times collision is checked per frame.
pub const NUM_TIMES: u32 = 32;
pub const NUM_TIMES_F64: f64 = NUM_TIMES as f64;
pub const CONVEYOR_STRENTH: f64 = 0.02 * GRID_SIZE / NUM_TIMES_F64; // how strong a conveyor belt is

pub const NUM_PARTITIONS: u32 = 8; // we will partition the map into 8*8=64 parts. A maximum of 64 partitions is supported.
