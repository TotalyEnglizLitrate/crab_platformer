use std::collections::HashMap;

use raylib::prelude::*;

use crate::state::StateTrait;
pub struct GameState;

impl StateTrait for GameState {
    fn bindings(&self) -> std::collections::HashMap<KeyboardKey, crate::state::Action> {
        HashMap::new()
    }
    fn update(&mut self, rl: &RaylibHandle) {}
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("game", 100, 100, 30, Color::WHITE);
    }
}
