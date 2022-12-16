use piston::Key;

use crate::{consts::{NUM_TIMES_F64, objects::{PLAYER_L_INDICATOR, PLAYER_R_INDICATOR, PLAYER_U_INDICATOR, PLAYER_D_INDICATOR}, PLAYER_SPEED_X, PLAYER_SPEED_Y}, render::{RenderJob, composite::Composite}, input::InputVars};

use super::object::Object;
pub struct Controls {
    pub horizontal_direction: f64, // left (-1), right (1), or N/A (0)
    pub vertical_direction: f64, // up (-1), down (1), or N/A (0)
    pub gravity_y: f64, // up (-1) or down (1). Cannot be changed while in the air. 
    pub gravity_x: f64, // up (-1), down (1), or neutral (0). Cannot be changed while in slime. 
    pub can_flip_x: bool,
    pub can_flip_y: bool,
}
impl Controls {
    pub fn new_level(&mut self, _: &mut Object, renderer: &mut RenderJob, _: &mut InputVars) {
        let composite = Composite::ensure_mut(renderer);
        composite.toggle_job(PLAYER_L_INDICATOR, false);
        composite.toggle_job(PLAYER_R_INDICATOR, false);
        composite.toggle_job(PLAYER_U_INDICATOR, false);
        composite.toggle_job(PLAYER_D_INDICATOR, false);
        self.horizontal_direction = 0.0;
        self.vertical_direction = 0.0;
    }
    pub fn left(&mut self, player: &mut Object, renderer: &mut RenderJob) {
        let composite = Composite::ensure_mut(renderer);
        composite.toggle_job(PLAYER_L_INDICATOR, true);
        composite.toggle_job(PLAYER_R_INDICATOR, false);
        self.horizontal_direction = -1.0;
    }
    pub fn right(&mut self, player: &mut Object, renderer: &mut RenderJob) {
        let composite = Composite::ensure_mut(renderer);
        composite.toggle_job(PLAYER_L_INDICATOR, false);
        composite.toggle_job(PLAYER_R_INDICATOR, true);
        self.horizontal_direction = 1.0;
    }
    pub fn up(&mut self, player: &mut Object, renderer: &mut RenderJob) {
        let composite = Composite::ensure_mut(renderer);
        composite.toggle_job(PLAYER_U_INDICATOR, true);
        composite.toggle_job(PLAYER_D_INDICATOR, false);
        self.vertical_direction = -1.0;
    }
    pub fn down(&mut self, player: &mut Object, renderer: &mut RenderJob) {
        let composite = Composite::ensure_mut(renderer);
        composite.toggle_job(PLAYER_U_INDICATOR, false);
        composite.toggle_job(PLAYER_D_INDICATOR, true);
        self.vertical_direction = 1.0;
    }
    pub fn space(&mut self, player: &mut Object, renderer: &mut RenderJob) {
        let composite = Composite::ensure_mut(renderer);
        composite.toggle_job(PLAYER_L_INDICATOR, false);
        composite.toggle_job(PLAYER_R_INDICATOR, false);
        composite.toggle_job(PLAYER_U_INDICATOR, false);
        composite.toggle_job(PLAYER_D_INDICATOR, false);
        self.horizontal_direction = 0.0;
        self.vertical_direction = 0.0;
    }
    pub fn update_player(&mut self, player: &mut Object, renderer: &mut RenderJob, input: &mut InputVars) {
        if input.key_pressed(Key::Left as u32) {
            self.left(player, renderer);
        }
        if input.key_pressed(Key::Right as u32) {
            self.right(player, renderer);
        }
        if input.key_pressed(Key::Up as u32) {
            self.up(player, renderer);
        }
        if input.key_pressed(Key::Down as u32) {
            self.down(player, renderer);
        }
        if input.key_pressed(Key::Space as u32) {
            self.space(player, renderer);
        }
        if self.can_flip_x {
            self.gravity_x = self.horizontal_direction;
        }
        player.x_speed += self.gravity_x * PLAYER_SPEED_X / NUM_TIMES_F64;
        if self.can_flip_y && self.vertical_direction != 0.0 {
            self.gravity_y = self.vertical_direction;
        }
        player.y_speed += self.gravity_y * PLAYER_SPEED_Y / NUM_TIMES_F64;
        self.can_flip_y = false;
        self.can_flip_x = true;
    }
    pub fn new() -> Controls {
        Controls { horizontal_direction: 0.0, vertical_direction: 0.0, gravity_y: 1.0, gravity_x: 0.0, can_flip_x: true, can_flip_y: false }
    }
}