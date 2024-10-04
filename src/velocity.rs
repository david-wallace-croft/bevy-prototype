use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
}
