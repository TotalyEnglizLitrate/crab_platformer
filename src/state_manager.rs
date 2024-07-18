use raylib::prelude::*;

use crate::state::{Action, StateTrait};

use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::menu_state::MenuState;

pub struct StateManager {
    pub current_state: Box<dyn StateTrait>,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            current_state: Box::new(MenuState {}),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.handle_keybinds(rl);
        self.current_state.update(rl);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.current_state.draw(d);
    }

    pub fn handle_keybinds(&mut self, rl: &RaylibHandle) -> () {
        for (keybind, action) in self.current_state.bindings() {
            if rl.is_key_pressed(keybind) {
                match action {
                    Action::NextState(state) => {
                        self.current_state = state;
                    }
                }
            }
        }
        ()
    }
}
