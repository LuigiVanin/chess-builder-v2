use uuid::Uuid;

use crate::{
  board::Board,
  shared::{
    strategy::{Move, PieceStrategy},
    utils::Direction,
  },
};

pub struct PieceKind {
  pub strategy: Box<dyn PieceStrategy>,
  code: String,
  notation: String,
  name: String,
  points: i32,
  id: usize,
}

impl Clone for PieceKind {
  fn clone(&self) -> Self {
    Self {
      strategy: self.strategy.clone(),
      code: self.code.clone(),
      notation: self.notation.clone(),
      name: self.name.clone(),
      points: self.points.clone(),
      id: self.id.clone(),
    }
  }
}

impl PartialEq for PieceKind {
  fn eq(&self, other: &Self) -> bool {
    self.code == other.code
      && self.notation == other.notation
      && self.name == other.name
      && self.points == other.points
      && self.id == other.id
  }
}

impl PieceKind {
  pub fn new(
    code: String,
    notation: String,
    name: String,
    points: i32,
    id: usize,
    strategy: Box<dyn PieceStrategy>,
  ) -> PieceKind {
    return PieceKind {
      code,
      notation,
      name,
      points,
      id,
      strategy,
    };
  }

  pub fn id(&self) -> usize {
    self.id
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn code(&self) -> &str {
    &self.code
  }

  pub fn notation(&self) -> &str {
    &self.notation
  }
}

pub struct Piece {
  name: String,

  uuid: Uuid,
  kind: PieceKind,

  pub direction: Direction,

  move_count: usize,
}

impl Clone for Piece {
  fn clone(&self) -> Self {
    Self {
      name: self.name.clone(),
      uuid: self.uuid.clone(),
      kind: self.kind.clone(),
      direction: self.direction.clone(),
      move_count: self.move_count,
    }
  }
}

impl Piece {
  pub fn new(name: String, uuid: Uuid, kind: PieceKind, direction: Direction) -> Self {
    Piece {
      name,
      uuid,
      kind,
      direction,
      move_count: 0,
    }
  }

  pub fn uuid(&self) -> Uuid {
    self.uuid
  }

  pub fn kind(&self) -> &PieceKind {
    &self.kind
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn moveset(&self, board: &Board) -> Vec<Move> {
    return self.kind.strategy.moveset(board, self);
  }
}
