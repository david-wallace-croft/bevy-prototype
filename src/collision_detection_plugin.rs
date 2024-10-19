use super::asteroid::Asteroid;
use super::collider::Collider;
use super::in_game_set::InGameSet;
use super::spaceship::Spaceship;
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
          .or_default()
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

  fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
  ) {
    for (entity, collider) in query.iter() {
      for &collided_entity in collider.colliding_entities.iter() {
        if query.get(collided_entity).is_ok() {
          continue;
        }

        commands.entity(entity).despawn_recursive();
      }
    }
  }
}

impl Plugin for CollisionDetectionPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app
      .add_systems(
        Update,
        CollisionDetectionPlugin::collision_detection
          .in_set(InGameSet::CollisionDetection),
      )
      .add_systems(
        Update,
        (
          Self::handle_collisions::<Asteroid>,
          Self::handle_collisions::<Spaceship>,
        )
          .in_set(InGameSet::DespawnEntities),
      );
  }
}
