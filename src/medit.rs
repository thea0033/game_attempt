use std::{path::PathBuf, fs, mem};

use piston::Key;
use serde::{Serialize, Deserialize};

use crate::{internals::levels::{LevelGrid, GridSpace}, render::{RenderJobs, RenderJobID, RenderJob}, input::InputVars, consts::{self, MEDIT_TILE_SIZE, CONTENT_LAYER, MEDIT_TILES}};

#[derive(Serialize, Deserialize, Clone)]
pub struct IOMap {
    grid: Vec<Vec<LevelGrid>>,
    player_start: [usize; 4],
    size: [usize; 2],
}
impl IOMap {
    pub fn new() -> IOMap {
        let mut grid = LevelGrid::new();
        grid.contents[0][0] = GridSpace::StartingLocation;
        IOMap { grid: vec![vec![grid]], player_start: [0; 4], size: [1, 1] }
    }
}
pub struct MapRenderer {
    pub grid: Vec<Vec<Option<RenderJobID>>>,
    pub others: Vec<RenderJobID>,
    pub guide: Vec<RenderJobID>,
    pub mouse_hover: RenderJobID,
}
impl MapRenderer {
    pub fn new(jobs: &mut RenderJobs) -> MapRenderer {
        let mut res = MapRenderer { 
            grid: Vec::new(), 
            others: Vec::new(), 
            guide: Vec::new(),
            mouse_hover: jobs.add_job(RenderJob::default(), consts::UI_LAYER)
        };
        res.init(jobs);
        res
    }
    pub fn init(&mut self, jobs: &mut RenderJobs) {
        // initializes the guide
    }
    pub fn clear(&mut self, jobs: &mut RenderJobs) {
        for line in mem::take(&mut self.grid).into_iter().flatten().flatten() {
            jobs.remove_job(line);
        }
        for line in mem::take(&mut self.others) {
            jobs.remove_job(line);
        }
    }
    pub fn load(&mut self, jobs: &mut RenderJobs, to_load: &LevelGrid) {
        self.clear(jobs);
        let mut res: Vec<Vec<Option<RenderJobID>>> = to_load.contents.iter().enumerate().map(|(x, line)| 
            line.iter().enumerate().map(|(y, item)| {
                let job = item.to_render_job(x, y, MEDIT_TILE_SIZE, [0.0, 0.0]);
                job.map(|x| jobs.add_job(x, CONTENT_LAYER))
            }
        ).collect()).collect();
        let mut others: Vec<RenderJobID> = to_load.others.iter().map(|template| template.object.job.as_ref().map(|x| {
            let mut new_x = x.clone();
            todo!()
        })).flatten().collect();
        self.grid = res;
        self.others = others;
    }
}
pub struct Map {
    grid: Vec<Vec<LevelGrid>>,
    current: [usize; 2],
    size: [usize; 2],
    player_start: [usize; 4],
    file_path: PathBuf,
    renderer: MapRenderer
}
/**
 * Keybinds - 
 *  del - clear this grid, replacing all grid spaces with empty spaces - DONE
 *  ctrl + del - remove the entire column where this grid is - DONE
 *  alt + del - remove the entire row where this grid is - DONE
 *  arrow - go that direction, shifting to the leftmost/rightmost/top/bottom grid of possible - DONE
 *  ctrl + arrow - creates a new row/column in that direction - DONE
 *  left click - replace the item on the grid with the current item selected. 
 *  right click - replace the current item selected with the item on the grid. 
 *  enter - exit typing mode (typing mode is automatically entered when a complicated block [i.e. text] is placed) - TODO
 *  shift + enter - create a line break inside typing mode - TODO
 *  ctrl + enter - save your work - DONE
 *  alt + enter - save your work and quit - DONE
 *  ctrl + esc - quit without saving - DONE
 */
impl Map {
    pub fn new(path: PathBuf, jobs: &mut RenderJobs) -> Map {
        let mut grid = LevelGrid::new();
        grid.contents[0][0] = GridSpace::StartingLocation;
        Map { 
            grid: vec![vec![grid]], 
            player_start: [0; 4], 
            size: [1, 1], 
            current: [0, 0], 
            file_path: path,
            renderer: MapRenderer::new(jobs),
        }
    }
    pub fn tick(&mut self, jobs: &mut RenderJobs, input: &mut InputVars) -> bool {
        if input.key_down(Key::LCtrl as u32) || input.key_down(Key::RCtrl as u32) {
            // ctrl + [key]
            if input.key_pressed(Key::Escape as u32) {
                self.info();
                return true;
            } else if input.key_pressed(Key::Return as u32) {
                self.info();
                if let Err(e) = self.save() {
                    println!("Saving failed: {}", e);
                }
            } else if input.key_pressed(Key::Up as u32) {
                self.grow_vert();
                self.info();
            } else if input.key_pressed(Key::Down as u32) {
                self.down();
                self.grow_vert();
                self.info();
            } else if input.key_pressed(Key::Left as u32) {
                self.grow_horizon();
                self.info();
            } else if input.key_pressed(Key::Right as u32) {
                self.right();
                self.grow_horizon();
                self.info();
            } else if input.key_pressed(Key::Backspace as u32) {
                self.shrink_horizon(jobs);
                self.info();
            }  
        } else if input.key_down(Key::RAlt as u32) || input.key_down(Key::LAlt as u32) {
            // alt + [key]
            if input.key_pressed(Key::Return as u32) {
                if let Err(e) = self.save() {
                    println!("Saving failed: {}", e);
                } else {
                    return true;
                }
                self.info();
            } else if input.key_pressed(Key::Backspace as u32) {
                self.shrink_vert(jobs);
                self.info();
            }
        } else if input.key_down(Key::LShift as u32) || input.key_down(Key::RShift as u32) {
            // shift + [key]
            if input.key_pressed(Key::Return as u32) {
                // TODO: inserts /n
                todo!();
            }
        } else {
            // [key]
            if input.key_pressed(Key::Backspace as u32) {
                self.grid[self.current[1]][self.current[0]] = LevelGrid::new();
                self.info();
            } else if input.key_pressed(Key::Return as u32) {
                // exits typing mode
                todo!()
            } else if input.key_pressed(Key::Up as u32) {
                self.up();
                self.info();
            } else if input.key_pressed(Key::Down as u32) {
                self.down();
                self.info();
            } else if input.key_pressed(Key::Left as u32) {
                self.left();
                self.info();
            } else if input.key_pressed(Key::Right as u32) {
                self.right();
                self.info();
            }
        }
        // TODO: Add mouse functionality (UGGH)
        return false;
    }
    // Loads the file into a path buffer. Returns true if successfully loaded. 
    pub fn reset(&mut self, jobs: &mut RenderJobs) {
        jobs.clear();
    }
    pub fn info(&self) {
        println!("Grid size: [{}, {}]", self.size[0], self.size[1]);
        println!("Player start: {:?}", self.player_start);
        println!("Current position: {:?}", self.current);
    }
    pub fn load(path: PathBuf, jobs: &mut RenderJobs) -> Result<Map, String> {
        let io_map: IOMap = bincode::deserialize(&fs::read(&path).map_err(|x| x.to_string())?).map_err(|x| x.to_string())?;
        Ok(
            Map { 
                grid: io_map.grid, 
                current: [0, 0], 
                size: io_map.size, 
                player_start: io_map.player_start, 
                file_path: path,
                renderer: MapRenderer::new(jobs),
            }
        )
    }
    // Attempts to save the file. Returns true if successfully saved. 
    pub fn save(&mut self) -> Result<(), String> {
        let io_map = self.to_io_map();
        let data = bincode::serialize(&io_map).map_err(|x| x.to_string())?;
        std::fs::write(self.file_path.clone(), data).map_err(|x| x.to_string())
    }
    pub fn to_io_map(&self) -> IOMap {
        IOMap { grid: self.grid.clone(), player_start: self.player_start, size: self.size }
    }
    pub fn up(&mut self) {
        if self.current[1] == 0 {
            self.current[1] = self.size[1];
        }
        self.current[1] -= 1;
    }
    pub fn down(&mut self) {
        self.current[1] += 1;
        if self.current[1] == self.size[1] {
            self.current[1] = 0;
        }
    }
    pub fn left(&mut self) {
        if self.current[0] == 0 {
            self.current[0] = self.size[0];
        }
        self.current[0] -= 1;
        
    }
    pub fn right(&mut self) {
        self.current[0] += 1;
        if self.current[0] == self.size[0] {
            self.current[0] = 0;
        }
    }
    pub fn grow_vert(&mut self) {
        self.grid.insert(self.current[1], vec![LevelGrid::new(); self.size[0]]);
        self.size[1] += 1;
    }
    pub fn shrink_vert(&mut self, jobs: &mut RenderJobs) {
        if self.size[1] == 1 {
            self.clear(jobs);
            return;
        }
        self.grid.remove(self.current[1]);
        self.size[1] -= 1;
        if self.size[1] == self.current[1] {
            self.current[1] -= 1;
        }
    }
    pub fn grow_horizon(&mut self) {
        for line in &mut self.grid {
            line.insert(self.current[0], LevelGrid::new());
        }
        self.size[0] += 1;
    }
    pub fn shrink_horizon(&mut self, jobs: &mut RenderJobs) {
        if self.size[0] == 1 {
            self.clear(jobs);
            return;
        }
        for line in &mut self.grid {
            line.remove(self.current[0]);
        }
        self.size[0] -= 1;
        if self.size[0] == self.current[0] {
            self.current[0] -= 1;
        }
    }
    pub fn clear(&mut self, jobs: &mut RenderJobs) {
        self.current = [0, 0];
        self.grid = vec![vec![LevelGrid::new()]];
        self.grid[0][0].contents[0][0] = GridSpace::StartingLocation;
        self.player_start = [0; 4];
        self.size = [1, 1];
        self.renderer.clear(jobs);
    }
}
pub struct MousePos {
    raw: [f64; 2],
    grid_location: Option<[usize; 2]>,
    guide_location: Option<[usize; 2]>,
}
impl MousePos {
    pub fn get(raw: [f64; 2]) -> MousePos {
        let mouse_location_tiles = [raw[0] / MEDIT_TILE_SIZE, raw[1] / MEDIT_TILE_SIZE];
        let grid_location = if mouse_location_tiles[0] < (MEDIT_TILES as f64) && mouse_location_tiles[1] < (MEDIT_TILES as f64) {
            Some([mouse_location_tiles[0] as usize, mouse_location_tiles[1] as usize])
        } else {None};
        MousePos { raw, grid_location, guide_location: () }
    }
}