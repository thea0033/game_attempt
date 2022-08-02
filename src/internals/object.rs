use crate::{render::{RenderJobs, RenderJob, RenderJobID}, consts::{NUM_TIMES_F64, FUDGE, self, GRID_SIZE}};

use super::controls::Controls;

pub struct Object {
    pub x_pos: f64,
    pub y_pos: f64,
    pub x_speed: f64,
    pub y_speed: f64,
    pub job_id: RenderJobID,
    pub job: Option<RenderJob>,
    pub width: f64,
    pub height: f64,
}
impl Object {
    pub fn touching(b1: [f64; 4], b2: [f64; 4]) -> bool {
        return ( b1[0] < b2[2] ) && (b1[2] > b2[0]) && (b1[1] < b2[3]) && (b1[3] > b2[1]);
    }
    pub fn collides_bounds(&self, other_bounds: [f64; 4]) -> [bool; 4] {
        let this_bounds = [self.x_pos, self.y_pos, self.x_pos + self.width, self.y_pos + self.height];

        let top_strip = [other_bounds[0] + FUDGE, other_bounds[1] - FUDGE, other_bounds[2] - FUDGE, other_bounds[1] + FUDGE];
        let bottom_strip = [other_bounds[0] + FUDGE, other_bounds[3] + FUDGE, other_bounds[2] - FUDGE, other_bounds[3] + FUDGE];
        let left_strip = [other_bounds[0] - FUDGE, other_bounds[1] + FUDGE, other_bounds[0] - FUDGE, other_bounds[3] - FUDGE];
        let right_strip = [other_bounds[2] + FUDGE, other_bounds[1] + FUDGE, other_bounds[2] + FUDGE, other_bounds[3] - FUDGE];
        [Object::touching(this_bounds, top_strip), Object::touching(this_bounds, bottom_strip), Object::touching(this_bounds, left_strip), Object::touching(this_bounds, right_strip)]
    }
    // returns 
    pub fn collides(&self, other: &Object) -> [bool; 4] {
        let other_bounds = [other.x_pos, other.y_pos, other.x_pos + other.width, other.y_pos + other.height];
        // the top of the other object
        self.collides_bounds(other_bounds)
    }
    pub fn tick(&mut self, envs: &Vec<&Environment>, jobs: &mut RenderJobs) {
        self.x_pos += self.x_speed / NUM_TIMES_F64;
        self.y_pos += self.y_speed / NUM_TIMES_F64;
        for env in envs {
            if self.x_speed > 0.0 {
                self.x_speed -= env.x_friction / NUM_TIMES_F64;
                if self.x_speed < 0.0 {
                    self.x_speed = 0.0;
                }
            } else {
                self.x_speed += env.x_friction / NUM_TIMES_F64;
                if self.x_speed > 0.0 {
                    self.x_speed = 0.0;
                }
            }
            if self.y_speed > 0.0 {
                self.y_speed -= env.y_friction / NUM_TIMES_F64;
                if self.y_speed < 0.0 {
                    self.y_speed = 0.0;
                }
            } else {
                self.y_speed += env.y_friction / NUM_TIMES_F64;
                if self.y_speed > 0.0 {
                    self.y_speed = 0.0;
                }
            }
            self.x_speed *= (1.0 - env.x_drag).powf(1.0 / NUM_TIMES_F64);
            self.y_speed *= (1.0 - env.y_drag).powf(1.0 / NUM_TIMES_F64);
            self.x_speed += env.x_accel / NUM_TIMES_F64;
            self.y_speed += env.y_accel / NUM_TIMES_F64;
        }                
        let extracted_job = self.job.as_mut().xor(jobs.get_job_mut(self.job_id)).expect("safe unwrap");
        let bounds = extracted_job.bounds();
        bounds[0] = self.x_pos;
        bounds[1] = self.y_pos;
        bounds[2] = self.width;
        bounds[3] = self.height;
    }
    pub fn drop(self, jobs: &mut RenderJobs) {
        jobs.remove_job(self.job_id);
    }
}

pub struct Environment {
    // constant acceleration in the x and y direction that is applied to all objects in the environment
    pub x_accel: f64,
    pub y_accel: f64,
    // drag, between 0.0 and 1. At 0.0, velocity is preserved. At 1, the acceleration and speed are equal. 
    pub x_drag: f64,
    pub y_drag: f64,
    // constant friction, applied in the 0.0 direction. 
    pub x_friction: f64,
    pub y_friction: f64,
}
#[derive(Clone)]

pub struct ObjectTemplate {
    pub x_pos: Option<f64>,
    pub y_pos: Option<f64>,
    pub x_speed: Option<f64>,
    pub y_speed: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub job: Option<RenderJob>,
    pub layer: Option<u64>,
}
impl ObjectTemplate {
    pub fn new() -> ObjectTemplate {
        ObjectTemplate { x_pos: None, y_pos: None, x_speed: Some(0.0), y_speed: Some(0.0), width: None, height: None, job: None, layer: None }
    }
    pub fn x_pos(mut self, new: f64) -> Self {
        self.x_pos = Some(new);
        self
    }
    pub fn y_pos(mut self, new: f64) -> Self {
        self.y_pos = Some(new);
        self
    }
    pub fn x_speed(mut self, new: f64) -> Self {
        self.x_speed = Some(new);
        self
    }
    pub fn y_speed(mut self, new: f64) -> Self {
        self.y_speed = Some(new);
        self
    }
    pub fn width(mut self, new: f64) -> Self {
        self.width = Some(new);
        self
    }
    pub fn height(mut self, new: f64) -> Self {
        self.height = Some(new);
        self
    }
    pub fn job(mut self, new: RenderJob) -> Self {
        self.job = Some(new);
        self
    }
    pub fn layer(mut self, new: u64) -> Self {
        self.layer = Some(new);
        self
    }
    pub fn to_object(&self, jobs: &mut RenderJobs) -> Option<Object> {
        let mut other_job = self.job.clone()?;
        let bounds = other_job.bounds();
        bounds[0] = self.x_pos?;
        bounds[1] = self.y_pos?;
        bounds[2] = self.width?;
        bounds[3] = self.height?;
        let id = jobs.add_job(self.job.clone()?, self.layer?);
        Some(Object { 
            x_pos: self.x_pos?, 
            y_pos: self.y_pos?, 
            x_speed: self.x_speed?, 
            y_speed: self.y_speed?, 
            job_id: id, 
            job: None, 
            width: self.width?, 
            height: self.height?
        })
    }
    pub fn or(&mut self, other: &ObjectTemplate) {
        self.x_pos = self.x_pos.or(other.x_pos);
        self.y_pos = self.y_pos.or(other.y_pos);
        self.x_speed = self.x_speed.or(other.x_speed);
        self.y_speed = self.y_speed.or(other.y_speed);
        self.width = self.width.or(other.width);
        self.height = self.height.or(other.height);
        self.job = self.job.take().or(other.job.clone());
        self.layer = self.layer.or(other.layer);
    }
    pub fn update_object(&self, jobs: &mut RenderJobs, current: &mut Object, start_render: bool) {
        current.x_pos = self.x_pos.unwrap_or(current.x_pos);
        current.y_pos = self.y_pos.unwrap_or(current.y_pos);
        current.x_speed = self.x_speed.unwrap_or(current.x_speed);
        current.y_speed = self.y_speed.unwrap_or(current.y_speed);
        current.width = self.width.unwrap_or(current.width);
        current.height = self.height.unwrap_or(current.height);
        if let Some(job) = &self.job {
            if let Some(_) = current.job {
                // the current item is not being rendered. 
                if start_render {
                    current.job = None;
                    if let Some(layer) = self.layer {
                        jobs.add_job(job.clone(), layer);
                    } else {
                        jobs.set_job(job.clone(), current.job_id);
                    }
                } else {
                    current.job = Some(job.clone());
                }   
            } else {
                if let Some(layer) = self.layer {
                    jobs.add_job(job.clone(), layer);
                } else {
                    jobs.set_job(job.clone(), current.job_id);
                }
            }
        } else if let Some(layer) = self.layer {
            if start_render || current.job.is_none() {
                let extracted_job = current.job.take().xor(jobs.remove_job(current.job_id)).expect("safe unwrap");
                current.job_id = jobs.add_job(extracted_job, layer);
            } else {
                let extracted_job = current.job.take().expect("safe unwrap");
                current.job_id = jobs.add_job(extracted_job, layer);
                current.job = jobs.remove_job(current.job_id);
            }
        }
    }
}
#[derive(Clone)]
pub struct BlockTemplate {
    pub object: ObjectTemplate,
    pub behavior: Behavior,
}
impl BlockTemplate {
    pub fn new() -> BlockTemplate {
        BlockTemplate { object: ObjectTemplate::new(), behavior: Behavior::None }
    }
    pub fn to_block(&mut self, jobs: &mut RenderJobs) -> Option<Block> {
        if let Some(val) = self.object.to_object(jobs) {
            Some(Block::new(val, self.behavior))
        } else {
            None
        }
    }
    pub fn behavior(mut self, behavior: Behavior) -> Self {
        self.behavior = behavior;
        self
    }
}
pub enum CollideAction {
    Advance,
    Kill,
    MoveScreen(Direction),
    None
}
pub struct Block {
    pub object: Object,
    pub behavior: Behavior,
}
impl Block {
    pub fn shrinkage(&mut self) -> f64 {
        match self.behavior {
            Behavior::Stop => 0.0,
            Behavior::Kill => FUDGE,
            Behavior::Move(_) => 0.0,
            Behavior::Advance => FUDGE * 1.25,
            Behavior::Wrap => FUDGE,
            Behavior::Portal => FUDGE,
            Behavior::None => 0.0,
        }
    }
    pub fn collides(&mut self, player: &mut Object) -> [bool; 4] {
        let shrinkage = self.shrinkage();
        let other_bounds = [self.object.x_pos + shrinkage, self.object.y_pos + shrinkage, self.object.x_pos + self.object.width - shrinkage, self.object.y_pos + self.object.height - shrinkage];
        player.collides_bounds(other_bounds)
    }
    pub fn on_touch(&mut self, player: &mut Object, direction: Direction, ctrl: &mut Controls) -> CollideAction {
        match self.behavior {
            Behavior::Stop => match direction {
                Direction::Up => {
                    if player.y_speed > 0.0 { player.y_speed = 0.0;}
                    ctrl.can_flip = true;
                },
                Direction::Down => {
                    if player.y_speed < 0.0 { player.y_speed = 0.0}
                    ctrl.can_flip = true;
                },
                Direction::Left => if player.x_speed > 0.0 { player.x_speed = 0.0},
                Direction::Right => if player.x_speed < 0.0 { player.x_speed = 0.0},
            },
            Behavior::Kill => return CollideAction::Kill,
            // will eventually be a conveyor belt
            Behavior::Move(dir) => todo!(),
            Behavior::Advance => {return CollideAction::Advance},
            Behavior::Wrap => {
                match direction {
                    Direction::Up => player.y_pos = FUDGE,
                    Direction::Down => player.y_pos = consts::WINDOW_Y as f64 - GRID_SIZE - FUDGE,
                    Direction::Left => player.x_pos = FUDGE,
                    Direction::Right => player.x_pos = consts::WINDOW_X as f64 - GRID_SIZE - FUDGE,
                }
            },
            Behavior::Portal => {
                match direction {
                    Direction::Up => player.y_pos = FUDGE,
                    Direction::Down => player.y_pos = consts::WINDOW_Y as f64 - GRID_SIZE - FUDGE,
                    Direction::Left => player.x_pos = FUDGE,
                    Direction::Right => player.x_pos = consts::WINDOW_X as f64 - GRID_SIZE - FUDGE,
                }
                return CollideAction::MoveScreen(direction.opposite())
            },
            Behavior::None => {},
        }
        return CollideAction::None;
    }
    pub fn new(object: Object, behavior: Behavior) -> Block {
        Block { object, behavior }
    }
}
// how the block interacts with the player on touch
#[derive(Clone, Copy)]
pub enum Behavior {
    Stop,
    Kill,
    Move(Direction),
    Advance,
    Wrap,
    Portal,
    None,
}
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction{
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}