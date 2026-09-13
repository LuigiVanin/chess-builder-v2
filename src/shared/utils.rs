use std::ops::{Add, Div, Mul, Sub};

pub trait Numeric:
  Copy
  + PartialEq
  + PartialOrd
  + Add<Output = Self>
  + Sub<Output = Self>
  + Mul<Output = Self>
  + Div<Output = Self>
{
}

impl<T> Numeric for T where
  T: Copy
    + PartialEq
    + PartialOrd
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
{
}

#[derive(Clone, Copy, Debug)]
pub struct Vector2d<T>
where
  T: Numeric,
{
  pub x: T,
  pub y: T,
}

impl<T: Numeric> Add for Vector2d<T> {
  type Output = Vector2d<T>;

  fn add(self, rhs: Self) -> Vector2d<T> {
    return Vector2d {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    };
  }
}

#[derive(Clone, Copy)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}

impl Direction {
  pub fn to_vector2d(self) -> Vector2d<isize> {
    match self {
      Direction::Up => Vector2d { x: 0, y: 1 },
      Direction::Down => Vector2d { x: 0, y: -1 },
      Direction::Right => Vector2d { x: 1, y: 0 },
      Direction::Left => Vector2d { x: -1, y: 0 },
    }
  }

  pub fn to_tuple(self) -> (isize, isize) {
    match self {
      Direction::Up => (0, 1),
      Direction::Down => (0, -1),
      Direction::Right => (1, 0),
      Direction::Left => (0, 1),
    }
  }
}
