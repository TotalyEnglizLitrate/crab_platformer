use raylib::prelude::*;

use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::menu_state::MenuState;

pub trait StateTrait {
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&self, d: &mut RaylibDrawHandle);
}

pub enum State {
    Menu(MenuState),
    Level(LevelState),
    Game(GameState),
}

impl StateTrait for State {
    fn update(&mut self, rl: &RaylibHandle) {
        match self {
            State::Menu(state) => state.update(rl),
            State::Level(state) => state.update(rl),
            State::Game(state) => state.update(rl),
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        match self {
            State::Menu(state) => state.draw(d),
            State::Level(state) => state.draw(d),
            State::Game(state) => state.draw(d),
        }
    }
}
