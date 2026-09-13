use crate::{
  board::{Board, Tile},
  pieces::Piece,
  shared::utils::{Direction, Vector2d},
};

#[derive(Clone)]
pub enum MovementKind {
  Leap,
  Normal,
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Action {
  Move,
  Capture,
}

#[derive(Clone, Debug)]
pub struct Move {
  source: Vector2d<i64>,
  target: Vector2d<i64>,
  action: Action,
}

// impl std::fmt::Display for Move {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//   }
// }

impl Move {
  pub fn new(source: Vector2d<i64>, target: Vector2d<i64>, action: Action) -> Move {
    Move {
      source,
      target,
      action,
    }
  }

  pub fn source(&self) -> Vector2d<i64> {
    self.source
  }

  pub fn target(&self) -> Vector2d<i64> {
    self.target
  }

  pub fn action(&self) -> Action {
    self.action
  }
}

#[derive(Clone)]
pub struct Pattern {
  pub movement: Vector2d<i64>,
  pub repeat: bool,
  pub kind: MovementKind,
  pub action: Vec<Action>,
}

pub trait PieceStrategy {
  fn moveset(&self, board: &Board, piece: &Piece) -> Vec<Move>;
  fn unsized_clone(&self) -> Box<dyn PieceStrategy>;
}

impl Clone for Box<dyn PieceStrategy> {
  fn clone(&self) -> Self {
    self.unsized_clone()
  }
}
