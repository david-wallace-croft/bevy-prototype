use super::velocity::Velocity;
use ::bevy::prelude::*;

pub struct MovementPlugin;

impl MovementPlugin {
  fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
      transform.translation.x += velocity.value.x;

      transform.translation.y += velocity.value.y;

      transform.translation.z += velocity.value.z;
    }
  }
}

impl Plugin for MovementPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(Startup, MovementPlugin::update_position);
  }
}
