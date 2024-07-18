use raylib::prelude::*;

use crate::enums::State;
use crate::traits::StateTrait;

use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::menu_state::MenuState;

pub struct StateManager {
    pub current_state: Box<dyn StateTrait>,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            current_state: Box::new(MenuState::new()),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.current_state.update(rl);
        match self.current_state.go_next_state() {
            &State::Level => self.current_state = Box::new(LevelState::new()),
            &State::Menu => self.current_state = Box::new(MenuState::new()),
            &State::Game => self.current_state = Box::new(GameState::new()),
            &State::None => {}
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.current_state.draw(d);
    }
}
