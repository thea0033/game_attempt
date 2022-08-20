use graphics::color::YELLOW;

use crate::{internals::object::{ObjectTemplate, Environment, BlockTemplate, Behavior, Transform}, render::{rect::Rect, texture::{ImageRenderer, TextureID}, text::TextRenderer}};

use super::{GRID_SIZE, CONTENT_LAYER, WHITE, RED, UI_LAYER, WINDOW_X, WINDOW_Y, DEFAULT_FONT_ID, GREEN, TILES, MEDIT_TILE_SIZE};
pub const SPIKE_TX: TextureID = TextureID(0);
pub const GOAL_TX: TextureID = TextureID(1);
pub const WRAP_TX: TextureID = TextureID(2);
pub const TRANSITION_TX: TextureID = TextureID(3);
pub const CONVEYER_L_TX: TextureID = TextureID(4);
pub const CONVEYER_R_TX: TextureID = TextureID(5);
pub const GAME_TRANSFORM: Transform = Transform {
    tile_offset: [-1.0; 2],
    tile_size: [GRID_SIZE; 2],
};
pub const MEDIT_TRANSFORM: Transform = Transform {
    tile_offset: [0.0; 2],
    tile_size: [MEDIT_TILE_SIZE; 2],
};
pub const PLAYER: ObjectTemplate = ObjectTemplate {
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(0.98),
    height: Some(0.98),
    job: Some(Rect::new(GREEN, [0.0; 4])),
    layer: Some(CONTENT_LAYER),
};
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
pub const SPIKE: ObjectTemplate = ObjectTemplate {
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    job: Some(ImageRenderer::new([0.0; 4], RED, SPIKE_TX)),
    layer: Some(CONTENT_LAYER),
};
pub const GOAL: ObjectTemplate = ObjectTemplate {
    x_pos: None,
    y_pos: None,
    x_speed: Some(0.0),
    y_speed: Some(0.0),
    width: Some(1.0),
    height: Some(1.0),
    job: Some(ImageRenderer::new([0.0; 4], YELLOW, GOAL_TX)),
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
pub const DEATH_SCENE: &str = "B B B B B B B B B B B B B B B B B B B B B B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ P _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ G G G _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B B B B B B B B B B B B B B B B B B B B B B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ B
B B B B B B B B B B B B B B B B B B B B B B
";
pub const DEATH_TEXT: &str = "You died!";
pub const FIRST_LEVEL: &str = "B B B B B B B B B B B B B B B B B B B B B B
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ P _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
B B B B B B B B B B B B B B B _ _ _ _ _ _ B
B _ _ _ _ _ _ _ _ _ _ _ B _ _ _ _ _ _ _ _ W
B _ _ _ _ _ _ _ _ _ _ B _ _ B B B B B B B B
W _ _ _ _ _ _ _ _ _ _ _ _ B _ _ _ _ _ _ _ W
W _ _ _ _ _ S S S S S _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ G _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
W _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ W
B B B B B B B B B B B B B B B B B B B B B B
";
pub const DEATH_TEXT_OBJ: BlockTemplate = 
BlockTemplate {
    object: ObjectTemplate {
        x_pos: Some(0.0),
        y_pos: Some(0.0),
        x_speed: Some(0.0),
        y_speed: Some(0.0),
        width: Some(TILES as f64),
        height: Some(TILES as f64),
        job: Some(TextRenderer::new_ref(DEATH_TEXT, [0.0; 4], RED, GRID_SIZE as u32, 0, 0, DEFAULT_FONT_ID)),
        layer: Some(UI_LAYER),
    },
    behavior: Behavior::None
};
pub const PLAYER_THRUST_H: f64 = 1.0;
pub const PLAYER_THRUST_V: f64 = 1.0;
pub const PLAYER_GRAVITY: f64 = 2.0;
// levels
pub const PLAYER_START_DEFAULT_POS: [usize; 2] = [0, 0];