use raylib::prelude::*;
pub struct GameState {
    pub next_state: String,
}

impl GameState {
    pub fn update(&self, rl: &RaylibHandle) {}
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("game", 100, 100, 30, Color::WHITE);
    }
}
