use crate::states::game_state::GameVariant;
use crate::traits::StateTrait;

pub enum State {
    None,
    Menu,
    Level,
    Game(GameVariant),
}
