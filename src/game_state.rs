use ::bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GateState {
  GameOver,
  #[default]
  InGame,
  Paused,
}
