use ::bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CollisionDamage {
  pub amount: f32,
}

impl CollisionDamage {
  pub fn new(amount: f32) -> Self {
    Self {
      amount,
    }
  }
}
