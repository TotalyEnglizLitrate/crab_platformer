use crate::raylib::*;
use crate::{enums::State, traits::StateTrait};

pub enum GameVariant {
    Puzzle,
    Zen,
}

pub struct GameState {
    pub next_state: State,
    pub variant: GameVariant,
}

impl GameState {
    pub fn new(variant: GameVariant) -> Self {
        GameState {
            next_state: State::None,
            variant,
        }
    }
    pub fn handle_keys(self, keys: Vec<KeyboardKey>) -> () {
        //TODO: implement a similar structure for zen and puzzle mode to call them easily with the GameVariant Enum
        match self.variant {
            GameVariant::Puzzle => unimplemented!(),
            GameVariant::Zen => unimplemented!(),
        }
        ()
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
