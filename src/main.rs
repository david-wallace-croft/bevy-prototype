use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

#[derive(Resource)]
struct GreetTimer(Timer);

struct HelloPlugin;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

impl Plugin for HelloPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.insert_resource(GreetTimer(Timer::from_seconds(
      2.,
      TimerMode::Repeating,
    )));

    app.add_systems(Startup, add_people);

    app.add_systems(Update, (update_people, greet_people).chain());
  }
}

fn add_people(mut commands: Commands) {
  commands.spawn((Person, Name("Alpha Bravo".to_string())));
  commands.spawn((Person, Name("Charlie Delta".to_string())));
  commands.spawn((Person, Name("Echo Foxtrot".to_string())));
}

fn greet_people(
  query: Query<&Name, With<Person>>,
  time: Res<Time>,
  mut timer: ResMut<GreetTimer>,
) {
  if timer.0.tick(time.delta()).just_finished() {
    for name in &query {
      println!("hello {}!", name.0);
    }
  }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
  for mut name in &mut query {
    if name.0 == "Charlie Delta" {
      name.0 = "Golf Hotel".to_string();

      break;
    }
  }
}
