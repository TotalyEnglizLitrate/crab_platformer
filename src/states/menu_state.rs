use crate::raylib::*;
use crate::{enums::State, traits::StateTrait};

pub struct MenuState {
    pub next_state: State,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            next_state: State::None,
        }
    }
}

impl StateTrait for MenuState {
    fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            self.next_state = State::Level;
        }
    }
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Main menu\n\n\nPress TAB to go to level picker",
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
