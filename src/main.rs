mod acceleration;
mod camera_plugin;
mod debug_plugin;
mod movement_plugin;
mod moving_object_bundle;
mod spaceship_plugin;
mod velocity;

use self::camera_plugin::CameraPlugin;
use self::debug_plugin::DebugPlugin;
use self::movement_plugin::MovementPlugin;
use self::spaceship_plugin::SpaceshipPlugin;
use ::bevy::prelude::*;

fn main() {
  let clear_color = ClearColor(Color::srgb(0.1, 0., 0.15));

  let ambient_light = AmbientLight {
    color: Default::default(),
    brightness: 750.,
  };

  App::new()
    .insert_resource(clear_color)
    .insert_resource(ambient_light)
    .add_plugins(DefaultPlugins)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .run();
}
