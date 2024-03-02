use std::collections::HashSet;

pub struct InputVars {
    key_down: HashSet<u32>,
    key_changed: HashSet<u32>,
    mouse_pressed: u8,
    mouse_changed: u8,
    pub mouse_pos: [f64; 2],
    pub has_focus: bool,
    pub has_cursor: bool,
    pub dimensions: [f64; 2],
    pub text_buffer: String,
}
impl InputVars {
    pub fn new() -> InputVars {
        InputVars {
            key_down: HashSet::new(),
            key_changed: HashSet::new(),
            mouse_pressed: 0,
            mouse_changed: 0,
            mouse_pos: [0.0, 0.0],
            has_focus: true,
            has_cursor: true,
            dimensions: [0.0, 0.0],
            text_buffer: String::new(),
        }
    }
    // should happen every frame
    pub fn reset(&mut self) {
        self.key_changed.clear();
        self.mouse_changed = 0;
    }
    pub fn update_key(&mut self, code: u32, state: bool) {
        self.key_changed.insert(code);
        if state {
            self.key_down.insert(code);
        } else {
            self.key_down.remove(&code);
        }
    }
    pub fn key_down(&self, code: u32) -> bool {
        self.key_down.contains(&code)
    }
    pub fn key_pressed(&self, code: u32) -> bool {
        self.key_down.contains(&code) && self.key_changed.contains(&code)
    }
    pub fn key_released(&self, code: u32) -> bool {
        !self.key_down.contains(&code) && self.key_changed.contains(&code)
    }
    pub fn update_mouse(&mut self, code: u8, state: bool) {
        self.mouse_changed |= code;
        if state {
            self.mouse_pressed |= code;
        } else {
            self.mouse_pressed &= !code;
        }
    }
    pub fn mouse_down(&self, code: u8) -> bool {
        (self.mouse_pressed & code) != 0
    }
    pub fn mouse_pressed(&self, code: u8) -> bool {
        ((self.mouse_pressed & code) != 0) && ((self.mouse_changed & code) != 0)
    }
    pub fn mouse_released(&self, code: u8) -> bool {
        ((self.mouse_pressed & code) == 0) && ((self.mouse_changed & code) != 0)
    }
    pub fn focus(&mut self, input: bool) {
        self.has_focus = input;
    }
    pub fn cursor(&mut self, input: bool) {
        self.has_cursor = input;
    }
    pub fn update_dimension(&mut self, input: [f64; 2]) {
        self.dimensions = input;
    }
    pub fn grab_text(&mut self) -> String {
        std::mem::take(&mut self.text_buffer)
    }
}
