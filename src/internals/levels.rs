use graphics::color::{MAGENTA, YELLOW};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;

use crate::{
    consts::{
        self, objects::SPIKE_TX, BLUE, CONVEYOR_L_TX, CONVEYOR_R_TX, GOAL_TX, GREEN, RED,
        TRANSITION_TX, TRANSPARENT, TRANS_GREEN, TRANS_RED, WHITE, WRAP_TX,
    },
    medit::IOMap,
    render::{rect::Rect, texture::ImageRenderer, RenderJob},
};

use super::object::BlockTemplate;

pub struct Levels {
    pub levels: Vec<Level>,
    // any unique extra bits that a level has (such as tutorial text)
}
impl Levels {
    pub fn new() -> Levels {
        let paths = vec![
            "assets/levels/death",
            "assets/levels/l1",
            "assets/levels/l2",
            "assets/levels/l3",
        ];
        Levels {
            levels: paths
                .into_iter()
                .map(|x| {
                    from_slice::<IOMap>(&std::fs::read(x).expect("Error reading level!"))
                        .expect("Error parsing level!")
                        .into_level()
                })
                .collect(),
        }
    }
}
pub struct Level {
    pub grid: Vec<Vec<LevelGrid>>,
    pub player_start: [usize; 2],
}
impl Level {
    pub fn start(&self) -> &LevelGrid {
        &self.grid[self.player_start[1]][self.player_start[0]]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelGrid {
    pub contents: Vec<Vec<GridSpace>>,
    pub others: Vec<BlockTemplate>,
}
impl LevelGrid {
    pub fn new() -> LevelGrid {
        LevelGrid {
            contents: vec![vec![GridSpace::None; consts::TILES + 2]; consts::TILES + 2],
            others: Vec::new(),
        }
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
        LevelGrid {
            contents: res,
            others: Vec::new(),
        }
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
    ConveyorR,
    ConveyorL,
    Slime,
    Water,
    // Flips the player
    Flipper,
    // there is nothing here
    None,
}
impl GridSpace {
    pub const MAX: usize = 14;
    pub fn from_id(id: usize) -> GridSpace {
        match id {
            0 => GridSpace::None,
            1 => GridSpace::Block,
            2 => GridSpace::Spike,
            3 => GridSpace::Enemy,
            4 => GridSpace::Goal,
            5 => GridSpace::StartingLocation,
            6 => GridSpace::Transition,
            7 => GridSpace::Wrap,
            8 => GridSpace::StickyBlock,
            9 => GridSpace::ConveyorR,
            10 => GridSpace::ConveyorL,
            11 => GridSpace::Slime,
            12 => GridSpace::Water,
            13 => GridSpace::Flipper,
            _ => GridSpace::None,
        }
    }
    pub fn location(x: u32, y: u32, grid_size: f64, offset: [f64; 2]) -> [f64; 4] {
        [
            (x as f64) * grid_size + offset[0],
            (y as f64) * grid_size + offset[1],
            grid_size,
            grid_size,
        ]
    }
    pub fn job_generator(&self, bounds: [f64; 4], color: [f32; 4]) -> RenderJob {
        match self {
            GridSpace::Spike => ImageRenderer::new(bounds, color, SPIKE_TX),
            GridSpace::Goal => ImageRenderer::new(bounds, color, GOAL_TX),
            GridSpace::Transition => ImageRenderer::new(bounds, color, TRANSITION_TX),
            GridSpace::ConveyorR => ImageRenderer::new(bounds, color, CONVEYOR_R_TX),
            GridSpace::ConveyorL => ImageRenderer::new(bounds, color, CONVEYOR_L_TX),
            GridSpace::Block
            | GridSpace::Enemy
            | GridSpace::StartingLocation
            | GridSpace::Wrap
            | GridSpace::StickyBlock
            | GridSpace::None
            | GridSpace::Slime
            | GridSpace::Water
            | GridSpace::Flipper => Rect::new(color, bounds),
        }
    }
    pub fn to_render_job(&self) -> RenderJob {
        self.job_generator([0.0; 4], self.color())
    }
    pub fn alter_render_job(&self, job: &mut RenderJob) {
        *job = self.job_generator(*job.bounds(), self.color());
    }
    pub fn alter_render_job_mouse(&self, job: &mut RenderJob) {
        let bounds = *job.bounds();
        let mut color = self.color();
        color[3] = 0.5; // half opaque for mouse hovering
        *job = self.job_generator(bounds, color);
    }
    pub fn color(&self) -> [f32; 4] {
        match self {
            GridSpace::Block => WHITE,
            GridSpace::Spike => RED,
            GridSpace::Enemy => RED,
            GridSpace::Goal => YELLOW,
            GridSpace::StartingLocation => GREEN,
            GridSpace::Transition => YELLOW,
            GridSpace::Wrap => YELLOW,
            GridSpace::StickyBlock => MAGENTA,
            GridSpace::ConveyorR => YELLOW,
            GridSpace::ConveyorL => YELLOW,
            GridSpace::Slime => TRANS_GREEN,
            GridSpace::Water => BLUE,
            GridSpace::None => TRANSPARENT,
            GridSpace::Flipper => TRANS_RED,
        }
    }
}
