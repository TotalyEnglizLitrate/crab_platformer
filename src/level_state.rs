use raylib::prelude::*;

pub struct LevelState {
    pub next_state: String,
}

impl LevelState {
    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.next_state = String::from("game")
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Level Picker\n\nPress TAB to go to game",
            100,
            100,
            30,
            Color::WHITE,
        );
    }
}
