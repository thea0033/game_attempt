use graphics::{grid, color::{YELLOW, MAGENTA}};
use serde::{Serialize, Deserialize};

use crate::{consts::{DEATH_SCENE, FIRST_LEVEL, DEATH_TEXT_OBJ, PLAYER_START_DEFAULT_POS, self, WHITE, RED, objects::SPIKE_TX, GOAL_TX, GREEN, TRANSITION_TX, WRAP_TX, CONVEYER_L_TX, CONVEYER_R_TX}, render::{RenderJob, rect::Rect, texture::ImageRenderer}};

use super::{object::{BlockTemplate}};

pub struct Levels {
    pub levels: Vec<Level>,
    // any unique extra bits that a level has (such as tutorial text)
}
impl Levels {
    pub fn new() -> Levels {
        let mut levels = Vec::new();
        // currently uses consts instead of file reading; this should change soon. 
        levels.push(Level {
            grid: vec![
                vec![LevelGrid::from_str(DEATH_SCENE.to_string()).add_others(vec![DEATH_TEXT_OBJ])]
            ],
            player_start: PLAYER_START_DEFAULT_POS
        });
        levels.push(Level {
            grid: vec![vec![LevelGrid::from_str(FIRST_LEVEL.to_string())]],
            player_start: PLAYER_START_DEFAULT_POS
        });
        Levels {
            levels,
        }
    }
}
pub struct Level {
    pub grid: Vec<Vec<LevelGrid>>,
    pub player_start: [usize; 2]
}
impl Level {
    pub fn start(&self) -> &LevelGrid {
        &self.grid[self.player_start[0]][self.player_start[1]]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelGrid {
    pub contents: Vec<Vec<GridSpace>>,
    pub others: Vec<BlockTemplate>,
}
impl LevelGrid {
    pub fn new() -> LevelGrid {
        LevelGrid { contents: vec![vec![GridSpace::None; consts::TILES + 2]; consts::TILES + 2], others: Vec::new() }
    }
    // DEPRECATED
    pub fn from_str(contents: String) -> LevelGrid {
        let mut lines = 0;
        let mut res = vec![vec![]];
        for c in contents.chars() {
            match c {
                'B' => res[lines].push(GridSpace::Block),
                'S' => res[lines].push(GridSpace::Spike),
                'E' => res[lines].push(GridSpace::Enemy),
                'P' => res[lines].push(GridSpace::StartingLocation),
                'G' => res[lines].push(GridSpace::Goal),
                'W' => res[lines].push(GridSpace::Wrap),
                'T' => res[lines].push(GridSpace::Transition),
                '_' => res[lines].push(GridSpace::None),
                '\n' => {
                    res.push(Vec::new());
                    lines += 1;
                }
                _ => {}
            }
        }
        LevelGrid { contents: res, others: Vec::new() }
    }
    pub fn add_others(mut self, others: Vec<BlockTemplate>) -> Self {
        self.others = others;
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GridSpace {
    // stops the player from moving
    Block,
    // kills the player
    Spike,
    // kills the player, moves
    Enemy,
    // advances the player one level
    Goal,
    // a player starts here
    StartingLocation,
    // if the player goes here and is on the edge of the board, they move to another level. 
    Transition,
    // if the player goes here and is on the edge of the board, they move to the other side of THIS level. 
    Wrap,
    // This block is sticky. The player cannot move side to side while touching it downwards, and vice versa. 
    StickyBlock,
    // this block moves the player while it's on it. 
    ConveyerR,
    ConveyerL,
    // there is nothing here
    None,
}
impl GridSpace {
    pub fn to_render_job(&self, x: usize, y: usize, grid_size: f64, offset: [f64; 2]) -> Option<RenderJob> {
        let bounds = [
            (x as f64) * grid_size + offset[0], 
            (y as f64) * grid_size + offset[1], 
            (x as f64 + 1.0) * grid_size + offset[0], 
            (y as f64 + 1.0) * grid_size + offset[1]];
        match self {
            GridSpace::Block => Some(Rect::new(WHITE, bounds)),
            GridSpace::Spike => Some(ImageRenderer::new(bounds, RED, SPIKE_TX)),
            GridSpace::Enemy => Some(Rect::new(RED, bounds)),
            GridSpace::Goal => Some(ImageRenderer::new(bounds, YELLOW, GOAL_TX)),
            GridSpace::StartingLocation => Some(Rect::new(GREEN, bounds)),
            GridSpace::Transition => Some(ImageRenderer::new(bounds, YELLOW, TRANSITION_TX)),
            GridSpace::Wrap => Some(ImageRenderer::new(bounds, YELLOW, WRAP_TX)),
            GridSpace::StickyBlock => Some(Rect::new(MAGENTA, bounds)),
            GridSpace::ConveyerR => Some(ImageRenderer::new(bounds, YELLOW, CONVEYER_R_TX)),
            GridSpace::ConveyerL => Some(ImageRenderer::new(bounds, YELLOW, CONVEYER_L_TX)),
            GridSpace::None => None,
        }
    }
}