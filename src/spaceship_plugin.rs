use super::acceleration::Acceleration;
use super::moving_object_bundle::MovingObjectBundle;
use super::scene_assets::SceneAssets;
use super::spaceship::Spaceship;
use super::velocity::Velocity;
use ::bevy::prelude::*;

const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_SPEED: f32 = 25.;
const STARTING_ACCELERATION: Vec3 = Vec3::ZERO;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

pub struct SpaceshipPlugin;

impl SpaceshipPlugin {
  fn spaceship_movement_controls(
    button_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    time: Res<Time>,
  ) {
    let (mut transform, mut velocity) = query.single_mut();

    let mut rotation = 0.;

    let mut roll = 0.;

    let mut movement = 0.;

    if button_input.pressed(KeyCode::KeyD) {
      rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if button_input.pressed(KeyCode::KeyA) {
      rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if button_input.pressed(KeyCode::KeyS) {
      movement = -SPACESHIP_SPEED;
    } else if button_input.pressed(KeyCode::KeyW) {
      movement = SPACESHIP_SPEED;
    }

    if button_input.pressed(KeyCode::ShiftLeft) {
      roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if button_input.pressed(KeyCode::ShiftRight) {
      roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);

    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;
  }

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

    commands.spawn((spaceship_bundle, Spaceship));
  }
}

impl Plugin for SpaceshipPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app
      .add_systems(PostStartup, SpaceshipPlugin::spawn_spaceship)
      .add_systems(Update, SpaceshipPlugin::spaceship_movement_controls);
  }
}
