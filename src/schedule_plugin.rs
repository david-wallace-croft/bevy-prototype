use super::in_game_set::InGameSet;
use crate::game_state::GameState;
use ::bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app
      .configure_sets(
        Update,
        (
          InGameSet::DespawnEntities,
          InGameSet::UserInput,
          InGameSet::EntityUpdates,
          InGameSet::CollisionDetection,
        )
          .chain()
          .run_if(in_state(GameState::InGame)),
      )
      .add_systems(
        Update,
        apply_deferred
          .after(InGameSet::DespawnEntities)
          .before(InGameSet::UserInput),
      );
  }
}
