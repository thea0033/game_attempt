use piston::Key;

use crate::{consts::{PLAYER_THRUST_H, PLAYER_GRAVITY, NUM_TIMES_F64, objects::{PLAYER_L_INDICATOR, PLAYER_R_INDICATOR, PLAYER_U_INDICATOR, PLAYER_D_INDICATOR}}, render::{RenderJob, composite::Composite}, input::InputVars};

use super::object::Object;

pub struct Controls {
    pub horizontal_direction: f64, // left (-1), right (1), or N/A (0)
    pub vertical_direction: f64, // up (-1), down (1), or N/A (0)
    pub gravity: f64, // up (-1) or down (1). Cannot be changed while in the air. 
    pub can_flip: bool,
}
impl Controls {
    pub fn new_level(&mut self, player: &mut Object, renderer: &mut RenderJob, input: &mut InputVars) {
        let mut composite = Composite::ensure_mut(renderer);
        composite.toggle_job(PLAYER_L_INDICATOR, false);
        composite.toggle_job(PLAYER_R_INDICATOR, false);
        composite.toggle_job(PLAYER_U_INDICATOR, false);
        composite.toggle_job(PLAYER_D_INDICATOR, false);
        self.horizontal_direction = 0.0;
        self.vertical_direction = 0.0;
    }
    pub fn update_player(&mut self, player: &mut Object, renderer: &mut RenderJob, input: &mut InputVars) {
        let mut composite = Composite::ensure_mut(renderer);
        if input.key_pressed(Key::Left as u32) {
            composite.toggle_job(PLAYER_L_INDICATOR, true);
            composite.toggle_job(PLAYER_R_INDICATOR, false);
            self.horizontal_direction = -1.0;
        }
        if input.key_pressed(Key::Right as u32) {
            composite.toggle_job(PLAYER_L_INDICATOR, false);
            composite.toggle_job(PLAYER_R_INDICATOR, true);
            self.horizontal_direction = 1.0;
        }
        if input.key_pressed(Key::Up as u32) {
            composite.toggle_job(PLAYER_U_INDICATOR, true);
            composite.toggle_job(PLAYER_D_INDICATOR, false);
            self.vertical_direction = -1.0;
        }
        if input.key_pressed(Key::Down as u32) {
            composite.toggle_job(PLAYER_U_INDICATOR, false);
            composite.toggle_job(PLAYER_D_INDICATOR, true);
            self.vertical_direction = 1.0;
        }
        if input.key_pressed(Key::Space as u32) {
            composite.toggle_job(PLAYER_L_INDICATOR, false);
            composite.toggle_job(PLAYER_R_INDICATOR, false);
            composite.toggle_job(PLAYER_U_INDICATOR, false);
            composite.toggle_job(PLAYER_D_INDICATOR, false);
            self.horizontal_direction = 0.0;
            self.vertical_direction = 0.0;
        }
        player.x_speed += self.horizontal_direction * PLAYER_THRUST_H / NUM_TIMES_F64;
        if self.can_flip && self.vertical_direction != 0.0 {
            self.gravity = self.vertical_direction;
        }
        player.y_speed += self.gravity * PLAYER_GRAVITY / NUM_TIMES_F64;
        self.can_flip = false;
    }
    pub fn new() -> Controls {
        Controls { horizontal_direction: 0.0, vertical_direction: 0.0, gravity: 1.0, can_flip: false }
    }
}