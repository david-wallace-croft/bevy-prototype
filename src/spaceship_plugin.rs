use super::moving_object_bundle::MovingObjectBundle;
use super::velocity::Velocity;
use crate::acceleration::Acceleration;
use ::bevy::prelude::*;

const STARTING_ACCELERATION: Vec3 = Vec3::ZERO;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

pub struct SpaceshipPlugin;

impl SpaceshipPlugin {
  fn spawn_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
  ) {
    let acceleration = Acceleration::new(STARTING_ACCELERATION);

    let scene = asset_server.load("spaceship.glb#Scene0");

    let transform = Transform::from_translation(STARTING_TRANSLATION);

    let model = SceneBundle {
      scene,
      transform,
      ..default()
    };

    let velocity = Velocity::new(STARTING_VELOCITY);

    let spaceship_bundle = MovingObjectBundle {
      acceleration,
      model,
      velocity,
    };

    commands.spawn(spaceship_bundle);
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
