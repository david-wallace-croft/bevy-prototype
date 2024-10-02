use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
  fn build(
    &self,
    app: &mut App,
  ) {
    app.add_systems(Startup, add_people);
    app.add_systems(
      Update,
      (hello_world, (update_people, greet_people).chain()),
    );
  }
}

fn add_people(mut commands: Commands) {
  commands.spawn((Person, Name("Alpha Bravo".to_string())));
  commands.spawn((Person, Name("Charlie Delta".to_string())));
  commands.spawn((Person, Name("Echo Foxtrot".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
  for name in &query {
    println!("hello {}!", name.0);
  }
}

fn hello_world() {
  println!("Hello, World!");
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(HelloPlugin)
    .run();
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
  for mut name in &mut query {
    if name.0 == "Charlie Delta" {
      name.0 = "Golf Hotel".to_string();

      break;
    }
  }
}
