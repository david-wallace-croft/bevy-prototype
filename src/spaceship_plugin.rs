use super::acceleration::Acceleration;
use super::moving_object_bundle::MovingObjectBundle;
use super::scene_assets::SceneAssets;
use super::velocity::Velocity;
use ::bevy::prelude::*;

const STARTING_ACCELERATION: Vec3 = Vec3::ZERO;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

pub struct SpaceshipPlugin;

impl SpaceshipPlugin {
  fn spawn_spaceship(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
  ) {
    let acceleration = Acceleration::new(STARTING_ACCELERATION);

    let scene: Handle<Scene> = scene_assets.spaceship.clone();

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
    app.add_systems(PostStartup, SpaceshipPlugin::spawn_spaceship);
  }
}
