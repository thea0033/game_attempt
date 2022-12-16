use std::{path::{PathBuf, Path}, io::{stdin}};

use consts::{ASSETS_FOLDER, DEFAULT_FONT_PATH, MEDIT_WINDOW_X, MEDIT_WINDOW_Y, WINDOW_X, WINDOW_Y};
use internals::Game;
use medit::Map;
use opengl_graphics::{TextureSettings, Texture};
use render::Window;

mod internals;
mod render;
#[allow(dead_code)]
mod consts;
mod input;
pub mod medit;

fn main() {
    // each frame... 
    let mut args = std::env::args();
    let x:u32;
    let y:u32;
    let path = if args.len() > 2 {
        args.next();
        if args.next().unwrap() == "edit".to_string() {
            let path: String = args.next().expect("Safe unwrap");
            let path = PathBuf::try_from(path).expect("Please enter a valid path!");
            x = MEDIT_WINDOW_X;
            y = MEDIT_WINDOW_Y;
            Some(path)
        } else {
            x = WINDOW_X;
            y = WINDOW_Y;
            None
        }
    } else {
        x = WINDOW_X;
        y = WINDOW_Y; 
        None
    };
    
    // sets a font path
    let mut font_path = PathBuf::new();
    // adds to the font path
    font_path.push(ASSETS_FOLDER);
    font_path.push(DEFAULT_FONT_PATH);
    // creates a font
    let font = std::fs::read(&font_path).expect("Erroring out for now");
    // adds the glyphs 
    let glyphs = opengl_graphics::GlyphCache::from_bytes(&font, (), TextureSettings::new()).unwrap();
    // creates a new window based on that font
    let mut window = Window::new(vec![glyphs], x, y);
    window.textures.add(Texture::from_path(Path::new("assets\\images\\spikes.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\images\\goal.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\images\\wrap.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\images\\transition.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\images\\conveyorL.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\images\\conveyorR.png"), &TextureSettings::new()).expect("File not found!"));
    if let Some(path) = path {
        let mut map = Map::load(path.clone(), &mut window.jobs).unwrap_or_else(|_| {
            println!("Error loading map!");
            Map::new(path, &mut window.jobs)});
        while window.run_loop_iteration() {
            map.tick(&mut window.jobs, &mut window.input);
        }
    } else {
        let mut game = Game::new(&mut window.jobs);
        while window.run_loop_iteration() {
            game.tick(&mut window.jobs, &mut window.input);
        }
    }
}
// debug function
#[allow(dead_code)]
fn wait_for_input() {
    stdin().read_line(&mut String::new()).unwrap();
}