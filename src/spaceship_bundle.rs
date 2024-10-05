use super::velocity::Velocity;
use ::bevy::prelude::*;

#[derive(Bundle)]
struct SpaceshipBundle {
  model: SceneBundle,
  velocity: Velocity,
}
