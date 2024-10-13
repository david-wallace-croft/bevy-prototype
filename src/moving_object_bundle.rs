use super::acceleration::Acceleration;
use super::collider::Collider;
use super::velocity::Velocity;
use ::bevy::prelude::*;

#[derive(Bundle)]
pub struct MovingObjectBundle {
  pub acceleration: Acceleration,
  pub collider: Collider,
  pub model: SceneBundle,
  pub velocity: Velocity,
}
