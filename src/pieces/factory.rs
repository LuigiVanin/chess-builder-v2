use uuid::Uuid;

use crate::{
  pieces::{
    Piece, PieceKind,
    patterns::pawn_pattern,
    strategies::{
      BishopStrategy, KingStrategy, KnightStrategy, PawnStrategy, QueenStrategy, RookStrategy,
    },
  },
  shared::utils::Direction,
};

pub struct PieceFactory;

impl PieceFactory {
  #[allow(non_snake_case)]
  pub fn Pawn() -> Piece {
    Piece::new(
      String::from("Pawn"),
      Uuid::new_v4(),
      PieceKind::new(
        String::from("pawn"),
        String::from(""),
        String::from("Pawn"),
        1,
        1,
        Box::new(PawnStrategy {}),
      ),
      // direction is UP
      Direction::Up,
    )
  }

  #[allow(non_snake_case)]
  pub fn Queen() -> Piece {
    Piece::new(
      String::from("Queen"),
      Uuid::new_v4(),
      PieceKind::new(
        String::from("queen"),
        String::from("q"),
        String::from("Queen"),
        9,
        2,
        Box::new(QueenStrategy {}),
      ),
      // direction is UP
      Direction::Up,
    )
  }

  #[allow(non_snake_case)]
  pub fn King() -> Piece {
    Piece::new(
      String::from("King"),
      Uuid::new_v4(),
      PieceKind::new(
        String::from("king"),
        String::from("x"),
        String::from("King"),
        i32::MAX,
        9,
        Box::new(KingStrategy {
          pattern: pawn_pattern(),
        }),
      ), // direction is UP
      Direction::Up,
    )
  }

  #[allow(non_snake_case)]
  pub fn Rook() -> Piece {
    let rook = PieceKind::new(
      String::from("rook"),
      String::from("r"),
      String::from("Rook"),
      5,
      3,
      Box::new(RookStrategy {
        pattern: pawn_pattern(),
      }),
    );

    Piece::new(
      String::from("Rook"),
      Uuid::new_v4(),
      rook,
      // direction is UP
      Direction::Up,
    )
  }

  #[allow(non_snake_case)]
  pub fn Bishop() -> Piece {
    let bishop = PieceKind::new(
      String::from("bishop"),
      String::from("b"),
      String::from("Bishop"),
      3,
      4,
      Box::new(BishopStrategy {
        pattern: pawn_pattern(),
      }),
    );

    Piece::new(
      String::from("Bishop"),
      Uuid::new_v4(),
      bishop,
      // direction is UP
      Direction::Up,
    )
  }

  #[allow(non_snake_case)]
  pub fn Knight() -> Piece {
    let knight = PieceKind::new(
      String::from("knight"),
      String::from("k"),
      String::from("Knight"),
      3,
      5,
      Box::new(KnightStrategy {
        pattern: pawn_pattern(),
      }),
    );

    Piece::new(
      String::from("Knight"),
      Uuid::new_v4(),
      knight,
      // direction is UP
      Direction::Up,
    )
  }
}
