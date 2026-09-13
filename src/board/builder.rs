use std::collections::HashMap;

use uuid::Uuid;

use crate::board::{Board, Tile};
use crate::pieces::factory::PieceFactory;
use crate::pieces::{Piece, PieceKind};
use crate::shared::interfaces::Builder;
use crate::shared::utils::{Direction, Vector2d};
pub struct BoardBuilder {
  tiles: Vec<(Tile, Vector2d<i64>)>,
  references: HashMap<Uuid, Piece>,
  dim: Vector2d<usize>,
}

impl BoardBuilder {
  pub fn new(h: usize, w: usize) -> Self {
    BoardBuilder {
      dim: Vector2d { x: w, y: h },
      references: HashMap::new(),
      tiles: vec![],
    }
  }

  fn add_piece(&mut self, piece: Piece, x: i64, y: i64) -> () {
    let position = Vector2d { x, y };

    self
      .tiles
      .push((Tile::Has(piece.uuid()), position));

    self
      .references
      .insert(piece.uuid(), piece);
  }

  pub fn add_pawn(mut self, dir: Direction, x: i64, y: i64) -> Self {
    let mut piece = PieceFactory::Pawn();
    piece.direction = dir;

    self.add_piece(piece, x, y);

    self
  }

  pub fn add_queen(mut self, dir: Direction, x: i64, y: i64) -> Self {
    let mut piece = PieceFactory::Queen();
    piece.direction = dir;

    self.add_piece(piece, x, y);

    self
  }

  pub fn add_king(mut self, dir: Direction, x: i64, y: i64) -> Self {
    let mut piece = PieceFactory::King();
    piece.direction = dir;

    self.add_piece(piece, x, y);

    self
  }

  pub fn add_rook(mut self, dir: Direction, x: i64, y: i64) -> Self {
    let mut piece = PieceFactory::Rook();
    piece.direction = dir;

    self.add_piece(piece, x, y);

    self
  }

  pub fn add_bishop(mut self, dir: Direction, x: i64, y: i64) -> Self {
    let mut piece = PieceFactory::Bishop();
    piece.direction = dir;

    self.add_piece(piece, x, y);

    self
  }

  pub fn add_knight(mut self, dir: Direction, x: i64, y: i64) -> Self {
    let mut piece = PieceFactory::Knight();
    piece.direction = dir;

    self.add_piece(piece, x, y);

    self
  }

  pub fn add(mut self, kind: PieceKind, dir: Direction, x: i64, y: i64) -> Self {
    let name = kind.name().to_string();

    let piece = Piece::new(name, Uuid::new_v4(), kind, dir);

    self.add_piece(piece, x, y);

    self
  }
}

impl Builder<Board> for BoardBuilder {
  fn build(&self) -> Board {
    let mut board = Board::new(self.dim.y, self.dim.x);

    for tile in self.tiles.iter() {
      match tile.0 {
        Tile::Empty => {}
        Tile::Wall => {
          let _ = board.add_wall(tile.1);
        }
        Tile::Has(id) => {
          if let Some(piece) = self.references.get(&id) {
            _ = board.add_piece(piece.clone(), tile.1);
          }
        }
      }
    }

    return board;
  }
}
