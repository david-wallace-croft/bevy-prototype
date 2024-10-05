use ::bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.;

pub struct CameraPlugin;

impl CameraPlugin {
  fn spawn_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0., CAMERA_DISTANCE, 0.)
      .looking_at(Vec3::ZERO, Vec3::Z);

    let camera3d_bundle = Camera3dBundle {
      transform,
      ..default()
    };

    commands.spawn(camera3d_bundle);
  }
}

impl Plugin for CameraPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(Startup, CameraPlugin::spawn_camera);
  }
}
