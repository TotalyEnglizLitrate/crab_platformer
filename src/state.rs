use std::collections::HashMap;

use raylib::prelude::*;

use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::menu_state::MenuState;

pub enum Action {
    NextState(Box<dyn StateTrait>),
    // Other keybind actions here.
}

pub trait StateTrait {
    //TODO: Figure out how to make the HashMap static
    fn bindings(&self) -> HashMap<KeyboardKey, Action>;
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&self, d: &mut RaylibDrawHandle);
}
