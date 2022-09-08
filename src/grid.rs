use crate::components::{Position, Size};
use bevy::prelude::*;

pub const GRID_WIDTH: u16 = 10;
pub const GRID_HEIGHT: u16 = 10;

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        scale_sprite(transform.as_mut(), sprite_size, window);
    }
}

fn scale_sprite(transform: &mut Transform, sprite_size: &Size, window: &Window) {
    transform.scale = Vec3::new(
        sprite_size.width / GRID_WIDTH as f32 * window.width() as f32,
        sprite_size.height / GRID_HEIGHT as f32 * window.height() as f32,
        1.0,
    );
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        translate_position(transform.as_mut(), pos, window);
    }
}

fn convert(pos: f32, bound_window: f32, grid_side_lenght: f32) -> f32 {
    let tile_size = bound_window / grid_side_lenght;
    pos / grid_side_lenght * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

fn translate_position(transform: &mut Transform, pos: &Position, window: &Window) {
    transform.translation = Vec3::new(
        convert(pos.x as f32, window.width() as f32, GRID_WIDTH as f32),
        convert(pos.y as f32, window.height() as f32, GRID_HEIGHT as f32),
        0.0,
    );
}

#[cfg(test)]
mod test {
    use crate::components::Size;
    use bevy::window::WindowId;
    use raw_window_handle::{RawWindowHandle, WebHandle};

    use super::*;

    #[test]
    fn transform_has_correct_scale_for_window() {
        // Setup
        let expected_transform = Transform {
            scale: Vec3::new(20., 20., 1.),
            ..default()
        };
        let mut default_transform = Transform {
            scale: Vec3::new(2., 3., 4.),
            ..default()
        };
        let sprite_size = Size::square(1.);

        // Create window
        let mut descriptor = WindowDescriptor::default();
        descriptor.height = 200.;
        descriptor.width = 200.;
        let raw_window_handle = RawWindowHandle::Web(WebHandle::empty());
        let window = Window::new(
            WindowId::new(),
            &descriptor,
            200,
            200,
            1.,
            None,
            raw_window_handle,
        );

        // Apply scale
        scale_sprite(&mut default_transform, &sprite_size, &window);

        assert_eq!(default_transform, expected_transform);
    }

    #[test]
    fn convert_position_x_for_grid_width() {
        let x = convert(4., 400., GRID_WIDTH as f32);

        assert_eq!(x, -20.)
    }

    #[test]
    fn convert_position_y_for_grid_height() {
        let y = convert(5., 400., GRID_HEIGHT as f32);

        assert_eq!(y, 20.)
    }
    #[test]
    fn translate_position_to_window() {
        let position = Position { x: 2, y: 8 };
        let mut default_transform = Transform::default();
        let expected = Transform {
            translation: Vec3::new(-100., 140., 0.),
            ..default()
        };

        // Create window
        let mut descriptor = WindowDescriptor::default();
        descriptor.height = 400.;
        descriptor.width = 400.;
        let raw_window_handle = RawWindowHandle::Web(WebHandle::empty());
        let window = Window::new(
            WindowId::new(),
            &descriptor,
            400,
            400,
            1.,
            None,
            raw_window_handle,
        );

        // Apply translation
        translate_position(&mut default_transform, &position, &window);

        assert_eq!(default_transform, expected);
    }
}
