use std::collections::HashMap;

use raylib::prelude::*;

use crate::{
    level_state::LevelState,
    state::{Action, StateTrait},
};
pub struct MenuState {}

impl StateTrait for MenuState {
    fn bindings(&self) -> HashMap<KeyboardKey, Action> {
        vec![(
            KeyboardKey::KEY_TAB,
            Action::NextState(Box::new(LevelState)),
        )]
        .into_iter()
        .collect()
    }

    fn update(&mut self, rl: &RaylibHandle) {}
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Main menu\n\n\nPress TAB to go to level picker",
            100,
            100,
            30,
            Color::WHITE,
        );
    }
}
