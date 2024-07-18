use raylib::prelude::*;

use crate::traits::StateTrait;

pub enum State {
    None,
    Menu,
    Level,
    Game,
}
