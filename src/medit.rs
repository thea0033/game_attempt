use std::{fs, mem, path::PathBuf};

use piston::Key;
use serde::{Deserialize, Serialize};

use crate::{
    consts::{
        self, CONTENT_LAYER, LEFT_MOUSE, MEDIT_GUIDE_SIZE, MEDIT_TILES, MEDIT_TILE_SIZE,
        RIGHT_MOUSE,
    },
    input::InputVars,
    internals::levels::{GridSpace, Level, LevelGrid},
    render::{RenderJob, RenderJobID, RenderJobs},
};

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
        IOMap {
            grid: vec![vec![grid]],
            player_start: [0; 4],
            size: [1, 1],
        }
    }
    pub fn into_level(self) -> Level {
        Level {
            grid: self.grid,
            player_start: [self.player_start[1], self.player_start[0]],
        }
    }
}
pub struct MapRenderer {
    pub grid: Vec<Vec<RenderJobID>>,
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
            mouse_hover: jobs.add_job(RenderJob::default(), consts::UI_LAYER),
        };
        res.init(jobs);
        res
    }
    pub fn init(&mut self, jobs: &mut RenderJobs) {
        // initializes the guide/grid
        for i in 0..consts::MEDIT_TILES {
            let mut v: Vec<RenderJobID> = Vec::new();
            for j in 0..consts::MEDIT_TILES {
                let mut job = RenderJob::default();
                *job.bounds() = GridSpace::location(j, i, MEDIT_TILE_SIZE, [0.0; 2]);
                v.push(jobs.add_job(job, CONTENT_LAYER));
            }
            self.grid.push(v);
        }
        for i in 0..(GridSpace::MAX as u32) {
            let mut job = GridSpace::from_id(i as usize).to_render_job();
            *job.bounds() = GridSpace::location(
                consts::MEDIT_TILES + i % MEDIT_GUIDE_SIZE,
                i / MEDIT_GUIDE_SIZE,
                MEDIT_TILE_SIZE,
                [0.0; 2],
            );
            self.guide.push(jobs.add_job(job, consts::UI_LAYER));
        }
    }
    pub fn clear(&mut self, jobs: &mut RenderJobs) {
        for line in mem::take(&mut self.others) {
            jobs.remove_job(line);
        }
        for line in &mut self.grid {
            for tile in line {
                GridSpace::None.alter_render_job(jobs.get_job_mut(*tile).unwrap());
            }
        }
    }
    pub fn load(&mut self, jobs: &mut RenderJobs, to_load: &LevelGrid) {
        self.clear(jobs);
        for (id, tile) in self
            .grid
            .iter()
            .flatten()
            .zip(to_load.contents.iter().flatten())
        {
            tile.alter_render_job(jobs.get_job_mut(*id).unwrap());
        }
        let others: Vec<RenderJobID> = to_load
            .others
            .iter()
            .map(|template| {
                template.object.job.as_ref().map(|x| {
                    let _new_x = x.clone();
                    todo!()
                })
            })
            .flatten()
            .collect();
        self.others = others;
    }
    pub fn replace(&mut self, jobs: &mut RenderJobs, new_item: GridSpace, pos: [usize; 2]) {
        let id = self.grid[pos[0]][pos[1]];
        new_item.alter_render_job(jobs.get_job_mut(id).unwrap());
    }
}
pub struct Map {
    grid: Vec<Vec<LevelGrid>>,
    current: [usize; 2],
    size: [usize; 2],
    player_start: [usize; 4],
    file_path: PathBuf,
    renderer: MapRenderer,
    current_item: GridSpace,
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
            current_item: GridSpace::None,
        }
    }
    pub fn tick(&mut self, jobs: &mut RenderJobs, input: &mut InputVars) -> bool {
        let mouse_pos = MousePos::get(input.mouse_pos);

        if input.key_down(Key::LCtrl as u32) || input.key_down(Key::RCtrl as u32) {
            // ctrl + [key]
            if input.key_pressed(Key::Escape as u32) {
                self.info();
                return true;
            } else if input.key_pressed(Key::Return as u32) {
                if let Err(e) = self.save() {
                    println!("Saving failed: {}", e);
                } else {
                    println!("Saved!");
                }
            } else if input.key_pressed(Key::Up as u32) {
                self.grow_vert();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Down as u32) {
                self.grow_vert_plus();
                self.down();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Left as u32) {
                self.grow_horizon();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Right as u32) {
                self.grow_horizon_plus();
                self.right();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Backspace as u32) {
                self.shrink_horizon(jobs);
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
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
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
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
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Return as u32) {
                // exits typing mode
                todo!()
            } else if input.key_pressed(Key::Up as u32) {
                self.up();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Down as u32) {
                self.down();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Left as u32) {
                self.left();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            } else if input.key_pressed(Key::Right as u32) {
                self.right();
                self.renderer
                    .load(jobs, &self.grid[self.current[1]][self.current[0]]);
                self.info();
            }
        }
        if input.mouse_down(LEFT_MOUSE) {
            if let Some(position) = mouse_pos.grid_location {
                if let GridSpace::StartingLocation = self.current_item {
                    self.grid[self.player_start[1]][self.player_start[0]].contents
                        [self.player_start[2]][self.player_start[3]] = GridSpace::None;
                    if self.player_start[..2] == self.current {
                        self.renderer.replace(
                            jobs,
                            GridSpace::None,
                            [self.player_start[2], self.player_start[3]],
                        );
                    }
                    self.grid[self.current[1]][self.current[0]].contents[position[1]]
                        [position[0]] = GridSpace::StartingLocation;
                    self.player_start =
                        [self.current[0], self.current[1], position[1], position[0]];
                    self.renderer.replace(
                        jobs,
                        self.current_item.clone(),
                        [position[1], position[0]],
                    );
                } else {
                    self.grid[self.current[1]][self.current[0]].contents[position[1]]
                        [position[0]] = self.current_item.clone();
                    self.renderer.replace(
                        jobs,
                        self.current_item.clone(),
                        [position[1], position[0]],
                    );
                }
            } else if let Some(position) = mouse_pos.guide_location {
                let item_index = position[1] * (MEDIT_GUIDE_SIZE as usize) + position[0];
                self.current_item = GridSpace::from_id(item_index);
                self.current_item.alter_render_job_mouse(
                    &mut jobs.get_job_mut(self.renderer.mouse_hover).unwrap(),
                );
            }
        } else if input.mouse_down(RIGHT_MOUSE) {
            if let Some(position) = mouse_pos.grid_location {
                self.current_item = self.grid[self.current[1]][self.current[0]].contents
                    [position[1]][position[0]]
                    .clone();
                self.current_item.alter_render_job_mouse(
                    &mut jobs.get_job_mut(self.renderer.mouse_hover).unwrap(),
                );
            } else if let Some(position) = mouse_pos.guide_location {
                let item_index = position[1] * (MEDIT_GUIDE_SIZE as usize) + position[0];
                self.current_item = GridSpace::from_id(item_index);
                self.current_item.alter_render_job_mouse(
                    &mut jobs.get_job_mut(self.renderer.mouse_hover).unwrap(),
                );
            }
        }
        return false;
    }
    pub fn info(&self) {
        println!("Grid size: [{}, {}]", self.size[0], self.size[1]);
        println!("Player start: {:?}", self.player_start);
        println!("Current position: {:?}", self.current);
    }
    // Loads the file into a path buffer. Returns true if successfully loaded.
    pub fn load(path: PathBuf, jobs: &mut RenderJobs) -> Result<Map, String> {
        let io_map: IOMap = serde_json::from_slice(&fs::read(&path).map_err(|x| x.to_string())?)
            .map_err(|x| x.to_string())?;
        let mut map = Map {
            grid: io_map.grid,
            current: [0, 0],
            size: io_map.size,
            player_start: io_map.player_start,
            file_path: path,
            renderer: MapRenderer::new(jobs),
            current_item: GridSpace::None,
        };
        map.renderer
            .load(jobs, &map.grid[map.current[1]][map.current[0]]);
        Ok(map)
    }
    // Attempts to save the file. Returns true if successfully saved.
    pub fn save(&mut self) -> Result<(), String> {
        let io_map = self.to_io_map();
        let data = serde_json::to_string_pretty(&io_map).map_err(|x| x.to_string())?;
        std::fs::write(self.file_path.clone(), data).map_err(|x| x.to_string())
    }
    pub fn to_io_map(&self) -> IOMap {
        IOMap {
            grid: self.grid.clone(),
            player_start: self.player_start,
            size: self.size,
        }
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
        // might need to change to index 0 if width and height are wrong
        if self.player_start[1] >= self.current[1] {
            self.player_start[1] += 1;
        }
        self.grid
            .insert(self.current[1], vec![LevelGrid::new(); self.size[0]]);
        self.size[1] += 1;
    }
    pub fn grow_vert_plus(&mut self) {
        // might need to change to index 0 if width and height are wrong
        if self.player_start[1] >= self.current[1] + 1 {
            self.player_start[1] += 1;
        }
        self.grid
            .insert(self.current[1] + 1, vec![LevelGrid::new(); self.size[0]]);
        self.size[1] += 1;
    }
    pub fn shrink_vert(&mut self, jobs: &mut RenderJobs) {
        if self.size[1] == 1 {
            self.clear(jobs);
            return;
        }
        // might need to change to index 0 if width and height are wrong
        if self.player_start[1] == self.current[1] {
            self.player_start = [0; 4];
            self.grid[0][0].contents[0][0] = GridSpace::StartingLocation;
        } else if self.player_start[1] > self.current[1] {
            self.player_start[1] -= 1;
        }
        self.grid.remove(self.current[1]);
        self.size[1] -= 1;
        if self.size[1] == self.current[1] {
            self.current[1] -= 1;
        }
    }
    pub fn grow_horizon(&mut self) {
        // might need to change to index 1 if width and height are wrong
        if self.player_start[0] >= self.current[0] {
            self.player_start[0] += 1;
        }
        for line in &mut self.grid {
            line.insert(self.current[0], LevelGrid::new());
        }
        self.size[0] += 1;
    }
    pub fn grow_horizon_plus(&mut self) {
        // might need to change to index 1 if width and height are wrong
        if self.player_start[0] >= self.current[0] + 1 {
            self.player_start[0] += 1;
        }
        for line in &mut self.grid {
            line.insert(self.current[0] + 1, LevelGrid::new());
        }
        self.size[0] += 1;
    }
    pub fn shrink_horizon(&mut self, jobs: &mut RenderJobs) {
        // might need to change to index 1 if width and height are wrong
        if self.player_start[0] == self.current[0] {
            self.player_start = [0; 4];
            self.grid[0][0].contents[0][0] = GridSpace::StartingLocation;
        } else if self.player_start[0] > self.current[0] {
            self.player_start[0] -= 1;
        }
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
    _raw: [f64; 2],
    grid_location: Option<[usize; 2]>,
    guide_location: Option<[usize; 2]>,
}
impl MousePos {
    pub fn get(raw: [f64; 2]) -> MousePos {
        let mouse_location_tiles = [
            (raw[0] / MEDIT_TILE_SIZE).floor(),
            (raw[1] / MEDIT_TILE_SIZE).floor(),
        ];
        let grid_location = if mouse_location_tiles[0] < (MEDIT_TILES as f64)
            && mouse_location_tiles[1] < (MEDIT_TILES as f64)
        {
            Some([
                mouse_location_tiles[0] as usize,
                mouse_location_tiles[1] as usize,
            ])
        } else {
            None
        };
        let guide_location = if mouse_location_tiles[0] >= (MEDIT_TILES as f64)
            && mouse_location_tiles[1] < (MEDIT_TILES as f64)
            && mouse_location_tiles[0] < ((MEDIT_TILES as f64) + (MEDIT_GUIDE_SIZE as f64))
        {
            Some([
                mouse_location_tiles[0] as usize - MEDIT_TILES as usize,
                mouse_location_tiles[1] as usize,
            ])
        } else {
            None
        };
        MousePos {
            _raw: raw,
            grid_location,
            guide_location,
        }
    }
}
