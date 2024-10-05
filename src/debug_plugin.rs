use super::velocity::Velocity;
use ::bevy::prelude::*;

pub struct DebugPlugin;

impl DebugPlugin {
  fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
      info!(
        "Entity {:?} is at position {:?},",
        entity, transform.translation
      );
    }
  }
}

impl Plugin for DebugPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(Startup, DebugPlugin::print_position);
  }
}
