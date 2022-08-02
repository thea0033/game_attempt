use crate::consts::{PLAYER_THRUST_H, PLAYER_THRUST_V, PLAYER_GRAVITY, NUM_TIMES_F64};

use super::object::Object;

pub struct Controls {
    pub horizontal_direction: f64, // left (-1), right (1), or N/A (0)
    pub vertical_direction: f64, // up (-1), down (1), or N/A (0)
    pub gravity: f64, // up (-1) or down (1). Cannot be changed while in the air. 
    pub can_flip: bool,
}
impl Controls {
    pub fn update_player(&mut self, player: &mut Object) {
        player.x_speed += self.horizontal_direction * PLAYER_THRUST_H / NUM_TIMES_F64;
        player.y_speed += self.vertical_direction * PLAYER_THRUST_V / NUM_TIMES_F64;
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