use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Velocity {
  pub value: Vec3,
}

impl Velocity {
  pub fn new(
    x: f32,
    y: f32,
    z: f32,
  ) -> Self {
    Self {
      value: Vec3 {
        x,
        y,
        z,
      },
    }
  }
}
