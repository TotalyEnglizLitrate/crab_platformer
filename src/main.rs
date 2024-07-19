#![allow(unused)]

use raylib::prelude as raylib;
mod actions;
mod enums;
mod puzzles;
mod state_manager;
mod states;
mod traits;
mod zen;

use crate::enums::State;
use crate::raylib::*;
use crate::state_manager::StateManager;

fn main() {
    let mut state_man = StateManager::new();

    let (mut rl, thread) = raylib::init().size(1100, 650).title("Hello, World").build();
    rl.set_target_fps(60u32);

    while !rl.window_should_close() {
        state_man.update(&rl);
        let mut d = &mut rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        state_man.draw(&mut d);
    }
}
