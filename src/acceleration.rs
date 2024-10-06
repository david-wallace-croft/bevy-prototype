use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Acceleration {
  pub value: Vec3,
}

impl Acceleration {
  pub fn new(value: Vec3) -> Self {
    Self {
      value,
    }
  }
}
