use ::bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct SceneAssets {
  pub asteroid: Handle<Scene>,
  pub missile: Handle<Scene>,
  pub spaceship: Handle<Scene>,
}
