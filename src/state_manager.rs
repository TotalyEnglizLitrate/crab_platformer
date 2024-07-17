use raylib::prelude::*;

use crate::enums::{State, StateTrait};

use crate::game_state::GameState;
use crate::level_state::LevelState;
use crate::menu_state::MenuState;

pub struct StateManager {
    pub current_state: State,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            current_state: State::Menu(MenuState {
                next_state: String::from("none"),
            }),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.current_state.update(rl);
        let next_state = match &self.current_state {
            State::Menu(menu_state) => &menu_state.next_state,
            State::Game(game_state) => &game_state.next_state,
            State::Level(level_state) => &level_state.next_state,
        };

        if next_state == &String::from("level picker") {
            self.current_state = State::Level(LevelState {
                next_state: String::from("none"),
            })
        } else if next_state == &String::from("game") {
            self.current_state = State::Game(GameState {
                next_state: String::from("none"),
            })
        } else if next_state == &String::from("menu") {
            self.current_state = State::Menu(MenuState {
                next_state: String::from("none"),
            })
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.current_state.draw(d);
    }
}
