use bevy::prelude::*;

mod snake;

use snake::{snake_movement, spawn_snake};

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_plugins(DefaultPlugins)
        .add_systems(Update, snake_movement)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
