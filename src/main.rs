use bevy::prelude::*;

mod snake;

fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, snake::spawn_system)
        .add_plugins(DefaultPlugins)
        .add_systems(Update, snake::movement_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
