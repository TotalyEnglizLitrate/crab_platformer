use crate::traits::StateTrait;

pub enum State {
    None,
    Menu,
    Level,
    Game(GameVariant),
}

#[derive(Clone, Copy)]
pub enum GameVariant {
    Puzzle,
    Zen,
}
