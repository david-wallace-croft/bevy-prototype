use super::acceleration::Acceleration;
use super::asteroid::Asteroid;
use super::collider::Collider;
use super::collision_damage::CollisionDamage;
use super::health::Health;
use super::scene_assets::SceneAssets;
use super::spawn_timer::SpawnTimer;
use super::velocity::Velocity;
use crate::game_state::GameState;
use ::bevy::prelude::*;
use ::rand::thread_rng;
use ::rand::Rng;
use ::std::ops::Range;
use ::std::time::Duration;

const ACCELERATION_SCALAR: f32 = 1.;
const COLLISION_DAMAGE: f32 = 35.;
const HEALTH: f32 = 80.;
const RADIUS: f32 = 2.5;
const ROTATE_SPEED: f32 = 2.5;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.;
const SPAWN_TIME_SECONDS: f32 = 1.;
const VELOCITY_SCALAR: f32 = 5.;

pub struct AsteroidPlugin;

impl AsteroidPlugin {
  fn rotate_asteroids(
    mut query: Query<&mut Transform, With<Asteroid>>,
    time: Res<Time>,
  ) {
    for mut transform in query.iter_mut() {
      transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
  }

  fn spawn_asteroid(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
  ) {
    let time_delta: Duration = time.delta();

    spawn_timer.timer.tick(time_delta);

    if !spawn_timer.timer.just_finished() {
      return;
    }

    let mut rng = thread_rng();

    let translation = Vec3::new(
      rng.gen_range(SPAWN_RANGE_X),
      0.,
      rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector = || {
      Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0))
        .normalize_or_zero()
    };

    let acceleration =
      Acceleration::new(random_unit_vector() * ACCELERATION_SCALAR);

    let collider = Collider::new(RADIUS);

    let scene_root = SceneRoot(scene_assets.asteroid.clone());

    let transform = Transform::from_translation(translation);

    let velocity = Velocity::new(random_unit_vector() * VELOCITY_SCALAR);

    let health = Health::new(HEALTH);

    let collision_damage = CollisionDamage::new(COLLISION_DAMAGE);

    commands.spawn((
      Asteroid,
      acceleration,
      collider,
      collision_damage,
      health,
      scene_root,
      transform,
      velocity,
    ));
  }
}

impl Plugin for AsteroidPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    let timer = Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating);

    let spawn_timer = SpawnTimer {
      timer,
    };

    app.insert_resource(spawn_timer).add_systems(
      PostUpdate,
      (
        AsteroidPlugin::spawn_asteroid,
        AsteroidPlugin::rotate_asteroids,
      )
        .chain()
        .run_if(in_state(GameState::InGame)),
    );
  }
}
