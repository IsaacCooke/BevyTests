mod inputs;
mod reactive_inputs;

use std::fmt;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::ui::AlignItems::Default;
use bevy::render::color::Color;
use crate::inputs::InputsPlugin;
use crate::reactive_inputs::ReactiveInputsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InputsPlugin, ReactiveInputsPlugin))
        // .add_systems(Startup, setup)
        // .add_systems(Update, square_color)
        .run();
}

#[derive(Component)]
struct Square;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
        .insert(Square);
}

fn square_color(
    mut interaction_query: Query<&mut Sprite, (Changed<Interaction>, With<Square>)>,
    mut square_query: Query<&mut Handle<ColorMaterial>, With<Square>>
) {
    for mut color in square_query.iter_mut() {
        for mut sprite in interaction_query.iter_mut() {
            *color = Handle::default();

            sprite.color = Color::rgb(0.0, 0.0, 1.0);
        }
    }
}

fn hello_world() {
    println!("Hello World!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// Plugin

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>, ) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

