use std::{path::{PathBuf, Path}, io::{stdin}};

use consts::{ASSETS_FOLDER, DEFAULT_FONT_PATH};
use internals::Game;
use medit::Map;
use opengl_graphics::{TextureSettings, Texture};
use piston::Key;
use render::Window;

mod internals;
mod render;
#[allow(dead_code)]
mod consts;
mod input;
pub mod medit;

fn main() {
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
    let mut window = Window::new(vec![glyphs]);

    window.textures.add(Texture::from_path(Path::new("assets\\spikes.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\goal.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\wrap.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\transition.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\conveyerL.png"), &TextureSettings::new()).expect("File not found!"));
    window.textures.add(Texture::from_path(Path::new("assets\\conveyerR.png"), &TextureSettings::new()).expect("File not found!"));
    // each frame... 
    let mut args = std::env::args();
    let path = if args.len() > 2 {
        args.next();
        if args.next().unwrap() == "edit".to_string() {
            let path: String = args.next().expect("Safe unwrap");
            let path = PathBuf::try_from(path).expect("Please enter a valid path!");
            Some(path)
        } else {
            None
        }
    } else { None };
    if let Some(path) = path {
        let mut map = Map::load(path.clone(), &mut window.jobs).unwrap_or_else(|_| Map::new(path, &mut window.jobs));
        while window.run_loop_iteration() {
            map.tick(&mut window.jobs, &mut window.input);
        }
    } else {
        let mut game = Game::new(&mut window.jobs);
        while window.run_loop_iteration() {
            game.tick(&mut window.jobs);
            if window.input.key_down(Key::Left as u32) {
                game.controls.horizontal_direction = -1.0;
            }
            if window.input.key_down(Key::Right as u32) {
                game.controls.horizontal_direction = 1.0;
            }
            if window.input.key_down(Key::Up as u32) {
                game.controls.vertical_direction = -1.0;
            }
            if window.input.key_down(Key::Down as u32) {
                game.controls.vertical_direction = 1.0;
            }
            if window.input.key_down(Key::Space as u32) {
                game.controls.horizontal_direction = 0.0;
                game.controls.vertical_direction = 0.0;
            }
        }
    }
}
// debug function
#[allow(dead_code)]
fn wait_for_input() {
    stdin().read_line(&mut String::new()).unwrap();
}