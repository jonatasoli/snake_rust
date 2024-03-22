use bevy::prelude::*;

use crate::components::GameEndEvent;

pub fn over_system(mut commands: Commands, mut reader: EventReader<GameEndEvent>) {
    if reader.read().next().is_some() {
        commands.spawn_empty().insert(GameEndEvent::GameOver);
        println!("{}", GameEndEvent::GameOver);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        components::Position,
        snake::{self, Head, LastTailPosition, Segments},
    };
    use bevy::app::App;

    #[test]
    fn game_end_event_with_game_over() {
        // Setup
        let mut app = App::new();

        // Sistemas
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>() // <--
            .add_systems(Startup, snake::spawn_system)
            .add_systems(Update, snake::movement_system)
            .add_systems(
                Update,
                snake::movement_input_system.before(snake::movement_system),
            )
            .add_systems(Update, over_system.after(snake::movement_system)); // <--

        // tecla para cima
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyW);
        app.insert_resource(input);

        // executgar sistema algumas vezes
        for _ in 0..3 {
            app.update(); // x: 5, y: 6
        }

        // Verificar que não há componente de game end
        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 0);

        for _ in 0..20 {
            app.update();
        }

        // Verificar que há componente de game end
        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 2);

        let mut query = app.world.query_filtered::<&Position, With<Head>>();
        let position_at_gameover = query.iter(&app.world).next().unwrap();
        let snake_position_after_game_over = position_at_gameover.clone();

        app.update();

        let mut query = app.world.query_filtered::<&Position, With<Head>>();
        let position_after_gameover = query.iter(&app.world).next().unwrap();

        assert_eq!(
            snake_position_after_game_over,
            position_after_gameover.clone()
        );
    }

    #[test]
    fn game_end_event_with_game_over_when_moving_left() {
        // Setup
        let mut app = App::new();

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>() // <--
            .add_systems(Startup, snake::spawn_system)
            .add_systems(Update, snake::movement_system)
            .add_systems(
                Update,
                snake::movement_input_system.before(snake::movement_system),
            )
            .add_systems(Update, over_system.after(snake::movement_system)); // <--

        // Add new input resource
        let mut input = ButtonInput::<KeyCode>::default();
        #[cfg(debug_assertions)]
        input.press(KeyCode::KeyA);
        #[cfg(not(debug_assertions))]
        input.press(KeyCode::ArrowLeft);
        app.insert_resource(input);

        // Run systems again
        for _ in 0..3 {
            app.update();
        }

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 0);

        for _ in 0..1 {
            app.update();
        }

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 1);
    }

    #[test]
    fn game_end_event_with_game_over_when_moving_right() {
        // Setup
        let mut app = App::new();

        // Add systems
        app.insert_resource(Segments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GameEndEvent>() // <--
            .add_systems(Startup, snake::spawn_system)
            .add_systems(Update, snake::movement_system)
            .add_systems(
                Update,
                snake::movement_input_system.before(snake::movement_system),
            )
            .add_systems(Update, over_system.after(snake::movement_system)); // <--

        // Add new input resource
        let mut input = ButtonInput::<KeyCode>::default();
        #[cfg(debug_assertions)]
        input.press(KeyCode::KeyD);
        #[cfg(not(debug_assertions))]
        input.press(KeyCode::ArrowRight);
        app.insert_resource(input);

        // Run systems again
        for _ in 0..7 {
            app.update();
        }

        let mut query = app.world.query::<&GameEndEvent>();
        assert_eq!(query.iter(&app.world).count(), 1);
    }
}
