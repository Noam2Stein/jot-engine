use super::*;

pub enum GameEvent {
    ExitRequested,
    Input(InputEvent),
}
