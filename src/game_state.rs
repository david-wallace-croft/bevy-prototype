use ::bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
  GameOver,
  #[default]
  InGame,
  Paused,
}
