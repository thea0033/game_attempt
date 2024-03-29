use graphics::color::{MAGENTA, YELLOW};

use crate::{
    internals::object::{Behavior, BlockTemplate, Environment, ObjectTemplate, Transform},
    render::{
        composite::Composite,
        rect::Rect,
        text::TextRenderer,
        texture::{ImageRenderer, TextureID},
    },
};

use super::{
    BLUE, CONTENT_LAYER, DEFAULT_FONT_ID, GREEN, GRID_SIZE, MAGNETA, MEDIT_TILE_SIZE, RED, TILES,
    TRANS_GREEN, TRANS_RED, UI_LAYER, WHITE,
};

// textures based on IDs
pub const SPIKE_TX: TextureID = TextureID(0);
pub const GOAL_TX: TextureID = TextureID(1);
pub const WRAP_TX: TextureID = TextureID(2);
pub const TRANSITION_TX: TextureID = TextureID(3);
pub const CONVEYOR_L_TX: TextureID = TextureID(4);
pub const CONVEYOR_R_TX: TextureID = TextureID(5);

pub const GAME_TRANSFORM: Transform = Transform {
    tile_offset: [-1.0; 2],
    tile_size: [GRID_SIZE; 2],
};

pub const MEDIT_TRANSFORM: Transform = Transform {
    tile_offset: [0.0; 2],
    tile_size: [MEDIT_TILE_SIZE; 2],
};

// left/right/up/down visual feedback that's all a part of the player
pub const PLAYER_L_INDICATOR: usize = 1;
pub const PLAYER_R_INDICATOR: usize = 2;
pub const PLAYER_U_INDICATOR: usize = 3;
pub const PLAYER_D_INDICATOR: usize = 4;

// creates a player object, and returns a template. 
pub fn player() -> ObjectTemplate {
    let mut player_job = Composite::new([0.0; 4], WHITE);
    let composite = Composite::ensure_mut(&mut player_job);
    composite.add_job(Rect::new(GREEN, [0.0, 0.0, 1.0, 1.0]), true); // the player's body
    composite.add_job(Rect::new(MAGNETA, [0.1, 0.45, 0.1, 0.1]), false); // leftward movement indicator
    composite.add_job(Rect::new(MAGNETA, [0.8, 0.45, 0.1, 0.1]), false); // rightward movement indicator
    composite.add_job(Rect::new(RED, [0.1, 0.05, 0.8, 0.05]), false); // upward movement indicator
    composite.add_job(Rect::new(RED, [0.1, 0.9, 0.8, 0.05]), false); // downward movement indicator
    ObjectTemplate {
        x_pos: None,
        y_pos: None,
        x_speed: Some(0.0),
        y_speed: Some(0.0),
        width: Some(0.98),
        height: Some(0.98),
        job: Some(player_job),
        layer: Some(UI_LAYER - 1),
    }
}
pub const BLOCK: ObjectTemplate = ObjectTemplate {
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    job: Some(Rect::new(WHITE, [0.0; 4])),
    layer: Some(CONTENT_LAYER),
};
pub const CONVEYOR_L: ObjectTemplate = ObjectTemplate {
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    job: Some(ImageRenderer::new([0.0; 4], WHITE, CONVEYOR_L_TX)),
    layer: Some(CONTENT_LAYER),
};
pub const CONVEYOR_R: ObjectTemplate = ObjectTemplate {
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    job: Some(ImageRenderer::new([0.0; 4], WHITE, CONVEYOR_R_TX)),
    layer: Some(CONTENT_LAYER),
};
pub const STICKY: ObjectTemplate = ObjectTemplate {
    job: Some(Rect::new(MAGENTA, [0.0; 4])),
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    layer: Some(CONTENT_LAYER),
};
pub const SLIME: ObjectTemplate = ObjectTemplate {
    job: Some(Rect::new(TRANS_GREEN, [0.0; 4])),
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    layer: Some(CONTENT_LAYER),
};
pub const WATER: ObjectTemplate = ObjectTemplate {
    job: Some(Rect::new(BLUE, [0.0; 4])),
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    layer: Some(CONTENT_LAYER),
};
pub const FLIPPER: ObjectTemplate = ObjectTemplate {
    job: Some(Rect::new(TRANS_RED, [0.0; 4])),
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    layer: Some(CONTENT_LAYER),
};
pub const SPIKE: ObjectTemplate = ObjectTemplate {
    job: Some(ImageRenderer::new([0.0; 4], RED, SPIKE_TX)),
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    layer: Some(CONTENT_LAYER),
};
pub const GOAL: ObjectTemplate = ObjectTemplate {
    job: Some(ImageRenderer::new([0.0; 4], YELLOW, GOAL_TX)),
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    layer: Some(CONTENT_LAYER),
};
pub const ENEMY: ObjectTemplate = ObjectTemplate {
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    job: Some(Rect::new(WHITE, [0.0; 4])),
    layer: Some(CONTENT_LAYER),
};
pub const PLAYER_ENV: Environment = Environment {
    x_accel: 0.0,
    y_accel: 0.0,
    x_drag: 0.2,
    y_drag: 0.2,
    x_friction: 0.1,
    y_friction: 0.1,
};
pub const DEATH_TEXT: &str = "You died!";
pub const DEATH_TEXT_OBJ: BlockTemplate = BlockTemplate {
    object: ObjectTemplate {
        x_pos: Some(0.0),
        y_pos: Some(0.0),
        x_speed: Some(0.0),
        y_speed: Some(0.0),
        width: Some(TILES as f64),
        height: Some(TILES as f64),
        job: Some(TextRenderer::new_ref(
            DEATH_TEXT,
            [0.0; 4],
            RED,
            GRID_SIZE as u32,
            0,
            0,
            DEFAULT_FONT_ID,
        )),
        layer: Some(UI_LAYER),
    },
    behavior: Behavior::None,
};
pub const PLAYER_GRAVITY: f64 = 2.0;
// levels
pub const PLAYER_START_DEFAULT_POS: [usize; 2] = [0, 0];

pub const PLAYER_SPEED_X: f64 = 0.06 * GRID_SIZE;
pub const PLAYER_SPEED_Y: f64 = 0.12 * GRID_SIZE;

pub const WATER_SPEED_MULTI: f64 = -0.1;
