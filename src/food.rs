use crate::{
    components::{Position, Size},
    grid::{GRID_HEIGHT, GRID_WIDTH},
};
use bevy::prelude::*;
use rand::prelude::random;

const FOOD_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct Food;

pub fn spawn_system(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<u16>() % GRID_WIDTH) as i32,
            y: (random::<u16>() % GRID_HEIGHT) as i32,
        })
        .insert(Size::square(0.8));
}

#[cfg(test)]
mod test {
    use crate::components::Position;

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn spawns_food_inplace(_execution in 0u32..1000) {
            // Setup app
            let mut app = App::new();

            // Add startup system
            app.add_startup_system(spawn_system);

            // Run systems
            app.update();

            let mut query = app.world.query_filtered::<&Position, With<Food>>();
            assert_eq!(query.iter(&app.world).count(), 1);
            query.iter(&app.world).for_each(|position| {
                let x = position.x;
                let y = position.y;

                assert!(x >= 0 && x as i32 <= (GRID_WIDTH -1) as i32);
                assert!(y >= 0 && y as i32 <= (GRID_HEIGHT -1) as i32);
            })
        }
    }
}
