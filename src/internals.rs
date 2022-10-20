pub mod object;
pub mod levels;
pub mod controls;
pub mod partition_map;

use std::{mem::take, collections::HashMap, hash::Hash};

use crate::{render::{RenderJobs}, consts::{BLOCK, SPIKE, PLAYER_ENV, NUM_TIMES, GOAL, GAME_TRANSFORM, WINDOW_Y, GRID_SIZE, WINDOW_X, FUDGE, player, STICKY, CONVEYOR_R, CONVEYOR_L, SLIME, WATER}, input::InputVars};

use self::{object::{Object, Environment, Block, Behavior, Direction, CollideAction, Transform}, levels::{GridSpace, Levels}, controls::Controls, partition_map::{PartitionMapID, PartitionMap, Partition}};


pub struct Game {
    // the contents of a level
    pub player: Option<Object>,
    pub partitioner: PartitionMap,
    // TODO: Any interactables that move must recalculate their partition every frame. 
    // This doesn't need to be done immediately due to the lack of interactables. 
    // Additionally: deleting interactables can be done by setting the entry to None. 
    pub interactables: Vec<Option<Block>>,
    pub non_interactables: Vec<Block>,
    pub player_env: Environment,
    pub current_level: usize,
    pub current_pos: [usize; 2],
    pub levels: Levels,
    pub controls: Controls,
    // etc
}
impl Game {
    pub fn new(jobs: &mut RenderJobs) -> Game {
        let mut game = Game {
            player: None,
            partitioner: PartitionMap::new(
            Partition {
                x: 0,
                y: 0,
            }),
            interactables: Vec::new(),
            non_interactables: Vec::new(),
            player_env: PLAYER_ENV,
            current_level: 1,
            levels: Levels::new(),
            controls: Controls::new(),
            current_pos: [0, 0],
        };
        game.new_level(jobs);
        game
    }
    pub fn new_level(&mut self, jobs: &mut RenderJobs) {
        self.drop_table_level(jobs);
        let level = &self.levels.levels[self.current_level];
        self.current_pos = [level.player_start[0], level.player_start[1]];
        let grid = &level.grid[level.player_start[1]][level.player_start[0]];
        for (i, line) in grid.contents.iter().enumerate() {
            for (j, block) in line.iter().enumerate() {
                let (template, behavior) = match block {
                    GridSpace::Block => (BLOCK, Behavior::Stop),
                    GridSpace::Spike => (SPIKE, Behavior::Kill),
                    GridSpace::Enemy => todo!(),
                    GridSpace::Goal => (GOAL, Behavior::Advance),
                    GridSpace::StartingLocation => {
                        self.player = Some(player().x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap());
                        continue;
                    },
                    GridSpace::None => continue,
                    GridSpace::Transition => (BLOCK, Behavior::Portal),
                    GridSpace::Wrap => (BLOCK, Behavior::Wrap),
                    GridSpace::StickyBlock => (STICKY, Behavior::Stick),
                    GridSpace::ConveyorR => (CONVEYOR_R, Behavior::Move(Direction::Right)),
                    GridSpace::ConveyorL => (CONVEYOR_L, Behavior::Move(Direction::Left)),
                    GridSpace::Slime => (SLIME, Behavior::Slime),
                    GridSpace::Water => (WATER, Behavior::Water),
                };
                let object = template.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap();
                let mut block = Block::new(object, behavior);
                block.object.tick(&Vec::new(), jobs);
                self.partitioner.add(block.object.partition);
                self.interactables.push(Some(block));
            }
        }
        for line in &grid.others {
            let block = line.clone().to_block(jobs, &GAME_TRANSFORM).unwrap();
            self.partitioner.add(block.object.partition);
            self.interactables.push(Some(block));
        }
    }
    pub fn dead(&mut self, jobs: &mut RenderJobs) {
        self.current_level = 0;
        self.new_level(jobs);
    }
    pub fn next_level(&mut self, jobs: &mut RenderJobs) {
        self.current_level += 1;
        self.new_level(jobs);
    }
    // does a reset
    pub fn drop_table_level(&mut self, jobs: &mut RenderJobs) {
        for line in take(&mut self.player) {
            line.drop(jobs);
        }
        for line in take(&mut self.interactables) {
            line.map(|x| x.object.drop(jobs));
        }
        self.controls = Controls::new();
        self.partitioner.clear();
    }
    pub fn drop_table(&mut self, jobs: &mut RenderJobs) {
        for line in take(&mut self.interactables) {
            line.map(|x| x.object.drop(jobs));
        }
        for line in take(&mut self.non_interactables) {
            line.object.drop(jobs);
        }
        self.partitioner.clear();
    }
    pub fn load_grid(&mut self, jobs: &mut RenderJobs) {
        self.drop_table(jobs);
        for (i, line) in self.levels.levels[self.current_level].grid[self.current_pos[0]][self.current_pos[1]].contents.iter().enumerate() {
            for (j, block) in line.iter().enumerate() {
                let (template, behavior) = match block {
                    GridSpace::Block => (BLOCK, Behavior::Stop),
                    GridSpace::Spike => (SPIKE, Behavior::Kill),
                    GridSpace::Enemy => todo!(),
                    GridSpace::Goal => (GOAL, Behavior::Advance),
                    GridSpace::StartingLocation | GridSpace::None => continue,
                    GridSpace::Transition => (BLOCK, Behavior::Portal),
                    GridSpace::Wrap => (BLOCK, Behavior::Wrap),
                    GridSpace::StickyBlock => (STICKY, Behavior::Stick),
                    GridSpace::ConveyorR => (CONVEYOR_R, Behavior::Move(Direction::Right)),
                    GridSpace::ConveyorL => (CONVEYOR_L, Behavior::Move(Direction::Left)),
                    GridSpace::Slime => (SLIME, Behavior::Slime),
                    GridSpace::Water => (WATER, Behavior::Water),
                };
                let object = template.x_pos(j as f64).y_pos(i as f64).to_object(jobs, &GAME_TRANSFORM).unwrap();
                let mut block = Block::new(object, behavior);
                block.object.tick(&Vec::new(), jobs);
                self.partitioner.add(block.object.partition);
                self.interactables.push(Some(block));
            }
        }
        for line in &self.levels.levels[self.current_level].start().others {
            let block = line.clone().to_block(jobs, &GAME_TRANSFORM).unwrap();
            if block.interactable() {
                self.partitioner.add(block.object.partition);
                self.interactables.push(Some(block));
            } else {
                self.non_interactables.push(block);
            }
        }
    }
    pub fn tick(&mut self, jobs: &mut RenderJobs, input: &mut InputVars) {
        // processes player collisions with blocks
        for _ in 0..NUM_TIMES {
            let player = self.player.as_mut().unwrap();
            player.tick(&vec![&self.player_env], jobs);
            self.partitioner.set_player(player.partition());
            let player_job = jobs.get_job_mut(player.job_id).expect("The player isn't rendered!");
            self.controls.update_player(player, player_job, input);
            let mut action_queue = Vec::new();
            // let mut counter = 0; // DEBUG
            for line in &self.partitioner.cache {
                // counter += 1;
                if let Some(block) = &mut self.interactables[line.0] {
                    let [up, down, left, right] = block.collides(player);
                    if up {
                        action_queue.push(block.on_touch(player, Direction::Up, &mut self.controls));
                    }
                    if down {
                        action_queue.push(block.on_touch(player, Direction::Down, &mut self.controls));
                    }
                    if left {
                        action_queue.push(block.on_touch(player, Direction::Left, &mut self.controls));
                    }
                    if right {
                        action_queue.push(block.on_touch(player, Direction::Right, &mut self.controls));
                    }
                }
            }
            // println!("NUMBER OF BLOCKS GONE THROUGH: {}", counter);
            // println!("TOTAL BLOCKS: {}", self.interactables.len());
            // println!("PLAYER PARTITION: {:?}", player.partition());
            let mut will_die: bool = false;
            let mut will_advance: bool = false;
            let mut will_move_screen: Option<Direction> = None;
            let mut will_wrap: Option<Direction> = None;
            for line in action_queue {
                match line {
                    CollideAction::Advance => will_advance = true,
                    CollideAction::Kill => will_die = true,
                    CollideAction::MoveScreen(dir) => will_move_screen = Some(dir),
                    CollideAction::Wrap(dir) => will_wrap = Some(dir),
                    CollideAction::None => (),
                }
            }
            {
                let _x = player; 
            };
            if will_die {
                self.dead(jobs);
            } else if will_advance {
                self.next_level(jobs);
            } else if let Some(dir) = will_move_screen {
                match dir {
                    Direction::Up => {
                        if self.current_pos[0] == 0 {
                            self.current_pos[0] = self.levels.levels[self.current_level].grid.len() - 1;
                        } else {
                            self.current_pos[0] -= 1;
                        }
                        self.player.as_mut().unwrap().y_pos = FUDGE;
                    },
                    Direction::Down => {
                        self.current_pos[0] += 1;
                        if self.current_pos[0] == self.levels.levels[self.current_level].grid.len() {
                            self.current_pos[0] = 0;
                        }
                        self.player.as_mut().unwrap().y_pos = (WINDOW_Y as f64) - GRID_SIZE - FUDGE;
                    },
                    Direction::Left => {
                        if self.current_pos[1] == 0 {
                            self.current_pos[1] = self.levels.levels[self.current_level].grid[0].len() - 1;
                        } else {
                            self.current_pos[1] -= 1;
                        }
                        self.player.as_mut().unwrap().x_pos = FUDGE;
                    },
                    Direction::Right => {
                        self.current_pos[1] += 1;
                        if self.current_pos[1] >= self.levels.levels[self.current_level].grid[0].len() {
                            self.current_pos[1] = 0;
                        }
                        self.player.as_mut().unwrap().x_pos = (WINDOW_X as f64) - GRID_SIZE - FUDGE;
                    },
                }
                self.load_grid(jobs);
            } else if let Some(dir) = will_wrap {
                match dir {
                    Direction::Up => {
                        self.player.as_mut().unwrap().y_pos = FUDGE;
                    },
                    Direction::Down => {
                        self.player.as_mut().unwrap().y_pos = (WINDOW_Y as f64) - GRID_SIZE - FUDGE;
                    },
                    Direction::Left => {
                        self.player.as_mut().unwrap().x_pos = FUDGE;
                    },
                    Direction::Right => {
                        self.player.as_mut().unwrap().x_pos = (WINDOW_X as f64) - GRID_SIZE - FUDGE;
                    },
                }
            }
        }
    }
}
