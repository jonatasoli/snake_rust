use bevy::prelude::{Component, Event};
use std::fmt::{self, Display};

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component, Clone, Debug, Event, PartialEq, Eq)]
pub enum GameEndEvent {
    GameOver,
}

impl Default for GameEndEvent {
    fn default() -> Self {
        Self::GameOver
    }
}

impl Display for GameEndEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameEndEvent::GameOver => write!(f, "Game Over!"),
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
    #[test]
    fn opposite_direction() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
    }
}
