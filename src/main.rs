use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

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
    .add_systems(Startup, add_people)
    .add_systems(Update, (greet_people, hello_world))
    .run();
}
