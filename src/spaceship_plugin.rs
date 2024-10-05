use super::velocity::Velocity;
use ::bevy::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

pub struct SpaceshipPlugin;

impl SpaceshipPlugin {
  fn spawn_spaceship(mut commands: Commands) {
    let spatial_bundle = SpatialBundle::default();

    let velocity = Velocity::new(STARTING_VELOCITY);

    commands.spawn((spatial_bundle, velocity));
  }
}

impl Plugin for SpaceshipPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(Startup, SpaceshipPlugin::spawn_spaceship);
  }
}
