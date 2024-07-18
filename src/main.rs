#![allow(unused)]

use raylib::prelude::*;
mod enums;
mod game_state;
mod level_state;
mod menu_state;
mod state_manager;

use crate::enums::State;

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
