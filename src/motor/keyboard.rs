use sdl2::keyboard::{KeyboardState, Keycode};
use std::collections::HashSet;

pub trait KeyboardListener {
    fn on_key_pressed(&mut self, key_code : Keycode);
}

pub struct MotorKeyboard {
    new_keys : HashSet<Keycode>,
    prev_keys : HashSet<Keycode>,

    listeners : Vec<Box<KeyboardListener>>


}

impl MotorKeyboard {
    pub fn new() -> MotorKeyboard {
        MotorKeyboard {
            new_keys : HashSet::new(),
            prev_keys : HashSet::new(),
            listeners : Vec::new()
        }
    }

    pub fn add_listener(&mut self, listener : Box<KeyboardListener>) {
        self.listeners.push(listener);
    }

    pub fn update(&mut self, keyboard_state : KeyboardState) {
        let keys = keyboard_state.pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        self.new_keys = &keys - &self.prev_keys;
        self.prev_keys = keys;
    }

    pub fn is_key_pressed(&self, key_code : Keycode) -> bool {
        return self.prev_keys.contains(&key_code);
    }

    pub fn is_key_just_pressed(&self, key_code : Keycode) -> bool {
        return self.new_keys.contains(&key_code);
    }

}
