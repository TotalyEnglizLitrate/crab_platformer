use crate::enums::State;
use crate::traits::StateTrait;
use raylib::prelude::*;

pub struct GameState {
    pub next_state: State,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            next_state: State::None,
        }
    }
}

impl StateTrait for GameState {
    fn update(&mut self, rl: &RaylibHandle) {}
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("game", 100, 100, 30, Color::WHITE);
    }
    fn go_next_state(&self) -> &State {
        &self.next_state
    }
}
