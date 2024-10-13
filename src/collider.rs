use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
  pub colliding_entities: Vec<Entity>,
  pub radius: f32,
}
