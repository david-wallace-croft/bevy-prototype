use super::acceleration::Acceleration;
use super::in_game_set::InGameSet;
use super::velocity::Velocity;
use ::bevy::prelude::*;

pub struct MovementPlugin;

impl MovementPlugin {
  fn update_position(
    mut query: Query<(&Velocity, &mut Transform)>,
    time: Res<Time>,
  ) {
    for (velocity, mut transform) in query.iter_mut() {
      transform.translation += velocity.value * time.delta_seconds();
    }
  }

  fn update_velocity(
    mut query: Query<(&Acceleration, &mut Velocity)>,
    time: Res<Time>,
  ) {
    for (acceleration, mut velocity) in query.iter_mut() {
      velocity.value += acceleration.value * time.delta_seconds();
    }
  }
}

impl Plugin for MovementPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(
      Update,
      (
        MovementPlugin::update_velocity,
        MovementPlugin::update_position,
      )
        .chain()
        .in_set(InGameSet::EntityUpdates),
    );
  }
}
