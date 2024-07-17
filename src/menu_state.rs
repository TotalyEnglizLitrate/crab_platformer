use raylib::prelude::*;
pub struct MenuState {
    pub next_state: String,
}

impl MenuState {
    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.next_state = String::from("level picker");
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Main menu\n\n\nPress TAB to go to level picker",
            100,
            100,
            30,
            Color::WHITE,
        );
    }
}
