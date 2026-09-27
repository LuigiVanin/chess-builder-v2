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
  Promote,
  Capture,
}

#[derive(Clone, Debug)]
pub struct Move {
  source: Vector2d<i64>,
  target: Vector2d<i64>,
  action: Vec<Action>,
}

// impl std::fmt::Display for Move {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//   }
// }

impl Move {
  pub fn new(source: Vector2d<i64>, target: Vector2d<i64>, action: Vec<Action>) -> Move {
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

  pub fn action(self) -> Vec<Action> {
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

pub fn mask_table_with_moves(mut table: Vec<Vec<String>>, moves: &Vec<Move>) -> Vec<Vec<String>> {
  for mov in moves {
    let actions = mov.clone().action();
    let mut character = String::from("0");

    for action in actions {
      character = match action {
        Action::Move => String::from("o"),
        Action::Capture => String::from("X"),
        Action::Promote => String::from("U"),
      }
    }
    table[mov.target().y as usize][mov.target().x as usize] = character;
  }

  return table;
}
