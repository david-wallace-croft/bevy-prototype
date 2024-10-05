mod debug_plugin;
mod movement_plugin;
mod spaceship_bundle;
mod spaceship_plugin;
mod velocity;

use self::debug_plugin::DebugPlugin;
use self::movement_plugin::MovementPlugin;
use self::spaceship_plugin::SpaceshipPlugin;
use ::bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .run();
}
