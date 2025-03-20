use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
  pub colliding_entities: Vec<Entity>,
  pub radius: f32,
}

impl Collider {
  pub fn new(radius: f32) -> Self {
    Self {
      colliding_entities: Vec::new(),
      radius,
    }
  }
}
