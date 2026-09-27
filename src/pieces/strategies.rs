use crate::{
  board::Board,
  pieces::{
    Piece,
    patterns::{
      bishop_pattern, king_pattern, knight_pattern, pawn_pattern, queen_pattern, rook_pattern,
    },
    utils::generate_moves,
  },
  shared::strategy::{Move, Pattern, PieceStrategy},
};

#[derive(Clone)]
pub struct PawnStrategy {
  // pub pattern: Vec<Pattern>,
}

#[derive(Clone)]
pub struct RookStrategy {
  pub pattern: Vec<Pattern>,
}

#[derive(Clone)]
pub struct BishopStrategy {
  pub pattern: Vec<Pattern>,
}

#[derive(Clone)]
pub struct KnightStrategy {
  pub pattern: Vec<Pattern>,
}

#[derive(Clone)]
pub struct KingStrategy {
  pub pattern: Vec<Pattern>,
}

#[derive(Clone)]
pub struct QueenStrategy {}

impl PieceStrategy for PawnStrategy {
  fn moveset(&self, board: &Board, piece: &Piece) -> Vec<Move> {
    return generate_moves(pawn_pattern(), board, piece);
  }

  fn unsized_clone(&self) -> Box<dyn PieceStrategy> {
    Box::new(self.clone())
  }
}

impl PieceStrategy for RookStrategy {
  fn moveset(&self, board: &Board, piece: &Piece) -> Vec<Move> {
    return generate_moves(rook_pattern(), board, piece);
  }

  fn unsized_clone(&self) -> Box<dyn PieceStrategy> {
    Box::new(self.clone())
  }
}

impl PieceStrategy for BishopStrategy {
  fn moveset(&self, board: &Board, piece: &Piece) -> Vec<Move> {
    generate_moves(bishop_pattern(), board, piece)
  }

  fn unsized_clone(&self) -> Box<dyn PieceStrategy> {
    Box::new(self.clone())
  }
}

impl PieceStrategy for KnightStrategy {
  fn moveset(&self, board: &Board, piece: &Piece) -> Vec<Move> {
    generate_moves(knight_pattern(), board, piece)
  }

  fn unsized_clone(&self) -> Box<dyn PieceStrategy> {
    Box::new(self.clone())
  }
}

impl PieceStrategy for QueenStrategy {
  fn moveset(&self, board: &Board, piece: &Piece) -> Vec<Move> {
    generate_moves(queen_pattern(), board, piece)
  }

  fn unsized_clone(&self) -> Box<dyn PieceStrategy> {
    Box::new(self.clone())
  }
}

impl PieceStrategy for KingStrategy {
  fn moveset(&self, board: &Board, piece: &Piece) -> Vec<Move> {
    generate_moves(king_pattern(), board, piece)
  }

  fn unsized_clone(&self) -> Box<dyn PieceStrategy> {
    Box::new(self.clone())
  }
}
