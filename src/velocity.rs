use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Velocity {
  pub value: Vec3,
}

impl Velocity {
  pub fn new(value: Vec3) -> Self {
    Self {
      value,
    }
  }
}
