#![allow(dead_code)]

mod acceleration;
mod asset_loader_plugin;
mod asteroid;
mod asteroid_plugin;
mod camera_plugin;
mod collider;
mod collision_damage;
mod collision_detection_plugin;
mod collision_event;
mod debug_plugin;
mod despawn_plugin;
mod game_state;
mod health;
mod in_game_set;
mod movement_plugin;
mod scene_assets;
mod schedule_plugin;
mod spaceship;
mod spaceship_missile;
mod spaceship_plugin;
mod spaceship_shield;
mod spawn_timer;
mod state_plugin;
mod velocity;

use self::asset_loader_plugin::AssetLoaderPlugin;
use self::asteroid_plugin::AsteroidPlugin;
use self::camera_plugin::CameraPlugin;
use self::collision_detection_plugin::CollisionDetectionPlugin;
use self::despawn_plugin::DespawnPlugin;
use self::movement_plugin::MovementPlugin;
use self::schedule_plugin::SchedulePlugin;
use self::spaceship_plugin::SpaceshipPlugin;
use self::state_plugin::StatePlugin;
use ::bevy::prelude::*;

fn main() {
  let clear_color = ClearColor(Color::srgb(0.1, 0., 0.15));

  let ambient_light = AmbientLight {
    affects_lightmapped_meshes: true,
    color: Default::default(),
    brightness: 1_000.,
  };

  App::new()
    .insert_resource(clear_color)
    .insert_resource(ambient_light)
    .add_plugins(DefaultPlugins)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(AsteroidPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(CollisionDetectionPlugin)
    .add_plugins(DespawnPlugin)
    .add_plugins(SchedulePlugin)
    .add_plugins(StatePlugin)
    .run();
}
