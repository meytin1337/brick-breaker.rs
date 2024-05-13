use bevy::prelude::*;

mod ellipse;

use crate::ellipse::shape::ShapePlugin;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin, ShapePlugin)).run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Hans Petter".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // println!("time: {:?}", time.delta());
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello, {}!", name.0);
        }
    }
}


fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Hans Petter" {
            name.0 = "Hans Petter Selasky".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}


