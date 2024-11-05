use super::acceleration::Acceleration;
use super::collider::Collider;
use super::in_game_set::InGameSet;
use super::moving_object_bundle::MovingObjectBundle;
use super::scene_assets::SceneAssets;
use super::spaceship::Spaceship;
use super::spaceship_missile::SpaceshipMissile;
use super::spaceship_shield::SpaceshipShield;
use super::velocity::Velocity;
use crate::collision_damage::CollisionDamage;
use crate::game_state::GameState;
use crate::health::Health;
use ::bevy::prelude::*;

const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_COLLISION_DAMAGE: f32 = 5.;
const MISSILE_HEALTH: f32 = 1.;
const MISSILE_RADIUS: f32 = 1.;
const MISSILE_SPEED: f32 = 50.;
const SPACESHIP_COLLISION_DAMAGE: f32 = 100.;
const SPACESHIP_HEALTH: f32 = 100.;
const SPACESHIP_RADIUS: f32 = 5.;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_SPEED: f32 = 25.;
const STARTING_ACCELERATION: Vec3 = Vec3::ZERO;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

pub struct SpaceshipPlugin;

impl SpaceshipPlugin {
  fn spaceship_destroyed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(), With<Spaceship>>,
  ) {
    if query.get_single().is_err() {
      next_state.set(GameState::GameOver)
    }
  }

  fn spaceship_movement_controls(
    button_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    time: Res<Time>,
  ) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
      return;
    };

    let mut rotation = 0.;

    let mut roll = 0.;

    let mut movement = 0.;

    if button_input.pressed(KeyCode::KeyD) {
      rotation = -SPACESHIP_ROTATION_SPEED * time.delta_secs();
    } else if button_input.pressed(KeyCode::KeyA) {
      rotation = SPACESHIP_ROTATION_SPEED * time.delta_secs();
    }

    if button_input.pressed(KeyCode::KeyS) {
      movement = -SPACESHIP_SPEED;
    } else if button_input.pressed(KeyCode::KeyW) {
      movement = SPACESHIP_SPEED;
    }

    if button_input.pressed(KeyCode::ShiftLeft) {
      roll = -SPACESHIP_ROLL_SPEED * time.delta_secs();
    } else if button_input.pressed(KeyCode::ShiftRight) {
      roll = SPACESHIP_ROLL_SPEED * time.delta_secs();
    }

    transform.rotate_y(rotation);

    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;
  }

  fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    button_input: Res<ButtonInput<KeyCode>>,
  ) {
    let Ok(spaceship) = query.get_single() else {
      return;
    };

    if button_input.pressed(KeyCode::Tab) {
      commands.entity(spaceship).insert(SpaceshipShield);
    }
  }

  fn spaceship_weapon_controls(
    button_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    scene_assets: Res<SceneAssets>,
  ) {
    if !button_input.pressed(KeyCode::Space) {
      return;
    }

    let acceleration = Acceleration::new(Vec3::ZERO);

    let collider = Collider::new(MISSILE_RADIUS);

    let scene: SceneRoot = SceneRoot(scene_assets.missile.clone());

    let transform = query.single();

    let model_translation: Vec3 = transform.translation
      + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR;

    let model_transform = Transform::from_translation(model_translation);

    let model = SceneBundle {
      scene,
      transform: model_transform,
      ..default()
    };

    let velocity = Velocity::new(-transform.forward() * MISSILE_SPEED);

    let moving_object_bundle = MovingObjectBundle {
      acceleration,
      collider,
      model,
      velocity,
    };

    let health = Health::new(MISSILE_HEALTH);

    let collision_damage = CollisionDamage::new(MISSILE_COLLISION_DAMAGE);

    commands.spawn((
      moving_object_bundle,
      SpaceshipMissile,
      health,
      collision_damage,
    ));
  }

  fn spawn_spaceship(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
  ) {
    let acceleration = Acceleration::new(STARTING_ACCELERATION);

    let collider = Collider::new(SPACESHIP_RADIUS);

    let scene: SceneRoot = SceneRoot(scene_assets.spaceship.clone());

    let transform = Transform::from_translation(STARTING_TRANSLATION);

    let model = SceneBundle {
      scene,
      transform,
      ..default()
    };

    let velocity = Velocity::new(STARTING_VELOCITY);

    let spaceship_bundle = MovingObjectBundle {
      acceleration,
      collider,
      model,
      velocity,
    };

    let health = Health::new(SPACESHIP_HEALTH);

    let collision_damage = CollisionDamage::new(SPACESHIP_COLLISION_DAMAGE);

    commands.spawn((spaceship_bundle, Spaceship, health, collision_damage));
  }
}

impl Plugin for SpaceshipPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app
      .add_systems(PostStartup, SpaceshipPlugin::spawn_spaceship)
      .add_systems(
        OnEnter(GameState::GameOver),
        SpaceshipPlugin::spawn_spaceship,
      )
      .add_systems(
        Update,
        (
          SpaceshipPlugin::spaceship_movement_controls,
          SpaceshipPlugin::spaceship_weapon_controls,
          SpaceshipPlugin::spaceship_shield_controls,
        )
          .chain()
          .in_set(InGameSet::UserInput),
      )
      .add_systems(
        Update,
        SpaceshipPlugin::spaceship_destroyed.in_set(InGameSet::EntityUpdates),
      );
  }
}
