use super::velocity::Velocity;
use crate::acceleration::Acceleration;
use ::bevy::prelude::*;

#[derive(Bundle)]
pub struct MovingObjectBundle {
  pub acceleration: Acceleration,
  pub model: SceneBundle,
  pub velocity: Velocity,
}
