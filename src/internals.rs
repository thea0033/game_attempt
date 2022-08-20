pub mod object;
pub mod levels;
pub mod controls;

use std::{mem::take};

use crate::{render::{RenderJobs}, consts::{PLAYER, BLOCK, SPIKE, ENEMY, PLAYER_ENV, NUM_TIMES, GRID_SIZE, GOAL, GAME_TRANSFORM}};

use self::{object::{Object, Environment, Block, Behavior, Direction, CollideAction}, levels::{GridSpace, Levels}, controls::Controls};


pub struct Game {
    // the contents of a level
    pub player: Option<Object>,
    pub blocks: Vec<Block>,
    pub player_env: Environment,

    pub current_level: usize,
    pub levels: Levels,
    pub controls: Controls,
    // etc
}
impl Game {
    pub fn new(jobs: &mut RenderJobs) -> Game {
        let mut game = Game { player: None,
            blocks: Vec::new(),
            player_env: PLAYER_ENV,
            current_level: 1,
            levels: Levels::new(),
            controls: Controls::new(),
        };
        game.load_grid(jobs);
        game
    }
    pub fn dead(&mut self, jobs: &mut RenderJobs) {
        self.current_level = 0;
        self.load_grid(jobs);
    }
    pub fn next_level(&mut self, jobs: &mut RenderJobs) {
        self.current_level += 1;
        self.load_grid(jobs);
    }
    // does a reset
    pub fn drop_table(&mut self, jobs: &mut RenderJobs) {
        for line in take(&mut self.player) {
            line.drop(jobs);
        }
        for line in take(&mut self.blocks) {
            line.object.drop(jobs);
        }
        self.controls = Controls::new();
    }
    pub fn load_grid(&mut self, jobs: &mut RenderJobs) {
        self.drop_table(jobs);
        for (i, line) in self.levels.levels[self.current_level].start().contents.iter().enumerate() {
            for (j, block) in line.iter().enumerate() {
                match block {
                    GridSpace::Block => {
                        let object = BLOCK.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap();
                        let block = Block::new(object, Behavior::Stop);
                        self.blocks.push(block);
                    },
                    GridSpace::Spike => {
                        let object = SPIKE.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap();
                        let spike = Block::new(object, Behavior::Kill);
                        self.blocks.push(spike);
                    },
                    GridSpace::Enemy => {
                        todo!()
                    },
                    GridSpace::Goal => {
                        let object = GOAL.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap();
                        let goal = Block::new(object, Behavior::Advance);
                        self.blocks.push(goal);
                    },
                    GridSpace::StartingLocation => {
                        self.player = Some(PLAYER.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap());
                    },
                    GridSpace::None => {},
                    GridSpace::Transition => {
                        let object = BLOCK.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap();
                        let transitioner = Block::new(object, Behavior::Portal);
                        self.blocks.push(transitioner);
                    },
                    GridSpace::Wrap => {
                        let object = BLOCK.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap();
                        let wrapper = Block::new(object, Behavior::Wrap);
                        self.blocks.push(wrapper);
                    },
                    GridSpace::StickyBlock => todo!(),
                    GridSpace::ConveyerR => todo!(),
                    GridSpace::ConveyerL => todo!(),
                }
            }
        }
        for line in &self.levels.levels[self.current_level].start().others {
            self.blocks.push(line.clone().to_block(jobs, &GAME_TRANSFORM).unwrap());
        }
    }
    pub fn tick(&mut self, jobs: &mut RenderJobs) {
        // processes player collisions with blocks
        for _ in 0..NUM_TIMES {
            self.player.as_mut().unwrap().tick(&vec![&self.player_env], jobs);
            self.controls.update_player(&mut self.player.as_mut().unwrap());
            let mut action_queue = Vec::new();
            for block in &mut self.blocks {
                block.object.tick(&Vec::new(), jobs);
                let [up, down, left, right] = block.collides(self.player.as_mut().unwrap());
                if up {
                    action_queue.push(block.on_touch(self.player.as_mut().unwrap(), Direction::Up, &mut self.controls));
                }
                if down {
                    action_queue.push(block.on_touch(self.player.as_mut().unwrap(), Direction::Down, &mut self.controls));
                }
                if left {
                    action_queue.push(block.on_touch(self.player.as_mut().unwrap(), Direction::Left, &mut self.controls));
                }
                if right {
                    action_queue.push(block.on_touch(self.player.as_mut().unwrap(), Direction::Right, &mut self.controls));
                }
            }
            let mut will_die: bool = false;
            let mut will_advance: bool = false;
            let mut will_move_screen: Option<Direction> = None;
            for line in action_queue {
                match line {
                    CollideAction::Advance => will_advance = true,
                    CollideAction::Kill => will_die = true,
                    CollideAction::MoveScreen(dir) => will_move_screen = Some(dir),
                    CollideAction::None => (),
                }
            }
            if will_die {
                self.dead(jobs);
            } else if will_advance {
                self.next_level(jobs);
            } else if let Some(dir) = will_move_screen {
                todo!()
            }
        }
    }
}
