use raylib::prelude::*;

use crate::enums::State;
use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::menu_state::MenuState;

pub trait StateTrait {
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn go_next_state(&self) -> &State;
}
