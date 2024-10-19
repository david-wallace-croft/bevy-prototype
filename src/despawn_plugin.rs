use super::in_game_set::InGameSet;
use ::bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.;

pub struct DespawnPlugin;

impl DespawnPlugin {
  fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform)>,
  ) {
    for (entity, transform) in query.iter() {
      let translation: Vec3 = transform.translation();

      let distance: f32 = translation.distance(Vec3::ZERO);

      if distance > DESPAWN_DISTANCE {
        commands.entity(entity).despawn_recursive();
      }
    }
  }
}

impl Plugin for DespawnPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(
      Update,
      DespawnPlugin::despawn_far_away_entities
        .in_set(InGameSet::DespawnEntities),
    );
  }
}