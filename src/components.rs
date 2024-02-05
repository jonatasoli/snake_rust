use bevy::prelude::Component;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Component, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    #[must_use]
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sized_square_is_created_calling_square_fn() {
        let expected = Size {
            width: 3.1,
            height: 3.1,
        };
        let actual = Size::square(3.1);

        assert_eq!(actual, expected);
    }
}
