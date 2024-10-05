use super::velocity::Velocity;
use ::bevy::prelude::*;

#[derive(Bundle)]
pub struct SpaceshipBundle {
  pub model: SceneBundle,
  pub velocity: Velocity,
}
