use super::acceleration::Acceleration;
use super::moving_object_bundle::MovingObjectBundle;
use super::spawn_timer::SpawnTimer;
use super::velocity::Velocity;
use crate::asteroid::Asteroid;
use ::bevy::prelude::*;
use ::rand::thread_rng;
use ::rand::Rng;
use ::std::ops::Range;
use ::std::time::Duration;

const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.;
const VELOCITY_SCALAR: f32 = 5.;

pub struct AsteroidPlugin;

impl AsteroidPlugin {
  fn spawn_asteroid(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
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

    let model: SceneBundle = SceneBundle {
      scene: asset_server.load("asteroid.glb#Scene0"),
      transform: Transform::from_translation(translation),
      ..default()
    };

    let velocity = Velocity::new(random_unit_vector() * VELOCITY_SCALAR);

    let moving_object_bundle = MovingObjectBundle {
      acceleration,
      model,
      velocity,
    };

    commands.spawn((moving_object_bundle, Asteroid));
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

    app
      .insert_resource(spawn_timer)
      .add_systems(Update, AsteroidPlugin::spawn_asteroid);
  }
}