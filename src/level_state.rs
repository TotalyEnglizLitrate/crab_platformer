use raylib::prelude::*;

use crate::{
    game_state::GameState,
    state::{Action, StateTrait},
};

pub struct LevelState;

impl StateTrait for LevelState {
    fn bindings(&self) -> std::collections::HashMap<KeyboardKey, crate::state::Action> {
        vec![(KeyboardKey::KEY_TAB, Action::NextState(Box::new(GameState)))]
            .into_iter()
            .collect()
    }
    fn update(&mut self, rl: &RaylibHandle) {}
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Level Picker\n\nPress TAB to go to game",
            100,
            100,
            30,
            Color::WHITE,
        );
    }
}
