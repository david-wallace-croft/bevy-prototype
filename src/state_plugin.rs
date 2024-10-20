use super::game_state::GameState;
use ::bevy::prelude::*;

pub struct StatePlugin;

impl StatePlugin {
  fn game_state_input_events(
    button_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
  ) {
    if button_input.just_pressed(KeyCode::Escape) {
      match state.get() {
        GameState::GameOver => (),
        GameState::InGame => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::InGame),
      }
    }
  }
}

impl Plugin for StatePlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app
      .init_state::<GameState>()
      .add_systems(Update, StatePlugin::game_state_input_events);
  }
}
