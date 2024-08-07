use crate::raylib::*;
use crate::{
    enums::{GameVariant, State},
    traits::StateTrait,
};

pub struct LevelState {
    pub next_state: State,
}

impl LevelState {
    pub fn new() -> Self {
        LevelState {
            next_state: State::None,
        }
    }
}

impl StateTrait for LevelState {
    fn update(&mut self, rl: &RaylibHandle) {
        //TODO: change this once buttons and/or keynav are added to reflect respective Game Variant
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.next_state = State::Game(GameVariant::Puzzle);
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Level Picker\n\nPress TAB to go to game",
            100,
            100,
            30,
            Color::WHITE,
        );
    }

    fn go_next_state(&self) -> &State {
        &self.next_state
    }
}
