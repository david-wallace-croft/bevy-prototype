mod velocity;

use self::velocity::Velocity;
use ::bevy::prelude::*;

fn main() {
  App::new()
    .add_systems(Startup, spawn_spaceship)
    .add_systems(Update, (update_position, print_position))
    .add_plugins(DefaultPlugins)
    .run();
}

fn print_position(query: Query<(Entity, &Transform)>) {
  for (entity, transform) in query.iter() {
    info!(
      "Entity {:?} is at position {:?},",
      entity, transform.translation
    );
  }
}

fn spawn_spaceship(mut commands: Commands) {
  let spatial_bundle = SpatialBundle::default();

  let velocity = Velocity::new(0.001, 0.001, 0.001);

  commands.spawn((spatial_bundle, velocity));
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
  for (velocity, mut position) in query.iter_mut() {
    position.translation.x += velocity.value.x;

    position.translation.y += velocity.value.y;

    position.translation.z += velocity.value.z;
  }
}
