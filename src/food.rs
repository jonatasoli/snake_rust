#[derive(Component)]
pub struct Food;

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
