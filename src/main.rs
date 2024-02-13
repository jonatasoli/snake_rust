use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod components;
pub mod food;
pub mod grid;
mod snake;

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, snake::spawn_system)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (500.0, 500.0).into(),
                        title: "Snake".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(PostUpdate, (grid::position_translation, grid::size_scaling))
        // .add_systems(Update, snake::movement_system)
        .add_systems(
            Update,
            food::spawn_system.run_if(on_timer(Duration::from_secs_f32(1.0))),
        )
        .add_systems(
            Update,
            snake::movement_system.run_if(on_timer(Duration::from_secs_f32(0.150))),
        )
        .add_systems(
            Update,
            snake::movement_input_system.before(snake::movement_system),
        )
        .add_systems(PostUpdate, (grid::position_translation, grid::size_scaling))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
