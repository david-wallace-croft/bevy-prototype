use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
  pub timer: Timer,
}
