use ::bevy::prelude::*;

#[derive(Debug, Message)]
pub struct CollisionEvent {
  pub collided_entity: Entity,
  pub entity: Entity,
}

impl CollisionEvent {
  pub fn new(
    collided_entity: Entity,
    entity: Entity,
  ) -> Self {
    CollisionEvent {
      collided_entity,
      entity,
    }
  }
}
