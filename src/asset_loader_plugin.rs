use super::scene_assets::SceneAssets;
use ::bevy::prelude::*;

pub struct AssetLoaderPlugin;

impl AssetLoaderPlugin {
  fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
  ) {
    *scene_assets = SceneAssets {
      asteroid: asset_server.load("asteroid.glb#Scene0"),
      missile: asset_server.load("missile.glb#Scene0"),
      spaceship: asset_server.load("spaceship.glb#Scene0"),
    };
  }
}

impl Plugin for AssetLoaderPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app
      .init_resource::<SceneAssets>()
      .add_systems(Startup, AssetLoaderPlugin::load_assets);
  }
}
