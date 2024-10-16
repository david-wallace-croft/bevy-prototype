use ::bevy::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub enum InGameSet {
  CollisionDetection,
  DespawnEntities,
  EntityUpdates,
  UserInput,
}
