use crate::{consts::{DEATH_SCENE, FIRST_LEVEL, DEATH_TEXT_OBJ, PLAYER_START_DEFAULT_POS}};

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
pub struct LevelGrid {
    pub contents: Vec<Vec<GridSpace>>,
    pub others: Vec<BlockTemplate>,
}
impl LevelGrid {
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