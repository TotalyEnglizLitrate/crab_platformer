use crate::enums::State;
use crate::raylib::*;

pub trait StateTrait {
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn go_next_state(&self) -> &State;
}
