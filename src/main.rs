use bevy::{prelude::*, time::FixedTimestep};
mod snake;
use snake::{movement_system, spawn_system};
pub mod components;
pub mod food;
pub mod grid;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake Game".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_system)
        .add_plugins(DefaultPlugins)
        .add_system(movement_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food::spawn_system),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(grid::position_translation)
                .with_system(grid::size_scaling),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
