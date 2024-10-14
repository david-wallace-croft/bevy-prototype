use super::collider::Collider;
use ::bevy::prelude::*;
use ::std::collections::HashMap;

pub struct CollisionDetectionPlugin;

impl CollisionDetectionPlugin {
  fn collision_detection(
    mut query: Query<(Entity, &GlobalTransform, &mut Collider)>
  ) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // TODO: This could probably be more efficient
    for (entity_a, transform_a, collider_a) in query.iter() {
      for (entity_b, transform_b, collider_b) in query.iter() {
        if entity_a == entity_b {
          continue;
        }

        let translation_a = transform_a.translation();

        let translation_b = transform_b.translation();

        let distance = translation_a.distance(translation_b);

        if distance >= collider_a.radius + collider_b.radius {
          continue;
        }

        colliding_entities
          .entry(entity_a)
          .or_insert_with(Vec::new)
          .push(entity_b);
      }
    }

    for (entity, _, mut collider) in query.iter_mut() {
      collider.colliding_entities.clear();

      if let Some(collisions) = colliding_entities.get(&entity) {
        collider
          .colliding_entities
          .extend(collisions.iter().copied())
      }
    }
  }
}

impl Plugin for CollisionDetectionPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(Update, CollisionDetectionPlugin::collision_detection);
  }
}
