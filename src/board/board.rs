use std::io::Empty;

use uuid::Uuid;

use crate::pieces::Piece;

// use super::pieces::Piece;
use crate::shared::utils::Vector2d;

#[derive(Debug)]
pub enum BoardError {
  OutOfBound,
  Unknow,
  AlreadyTaken,
  InvalidTile,
  PieceConflict,
  PieceNotFound,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Tile {
  Empty,
  Wall,
  Has(Uuid),
}

pub struct Board {
  table: Vec<Vec<Tile>>,
  pieces: Vec<Piece>,
}

impl Board {
  pub fn new(h: usize, w: usize) -> Board {
    Board {
      table: vec![vec![Tile::Empty; w]; h],
      pieces: vec![],
    }
  }

  pub fn tile_id(&self, tile: &Tile) -> String {
    match tile {
      Tile::Empty => "0".to_string(),
      Tile::Wall => "w".to_string(),
      Tile::Has(uuid) => {
        let piece = self
          .pieces
          .iter()
          .find(|p| p.uuid() == *uuid);

        match piece {
          None => "x".to_string(),
          Some(piece) => {
            let piece_id = piece.kind().id();
            format!("{}", piece_id)
          }
        }
      }
    }
  }

  pub fn dim(&self) -> Vector2d<usize> {
    return Vector2d {
      y: self.table.len(),
      x: self.table[0].len(),
    };
  }

  pub fn check_in_bounds(&self, x: i64, y: i64) -> bool {
    let dim = self.dim();

    #[allow(unused_comparisons)]
    if (x < 0 || x >= dim.x as i64) || (y < 0 || y >= dim.y as i64) {
      return false;
    }

    true
  }

  pub fn piece(&self, uuid: Uuid) -> Option<Piece> {
    let found = self.pieces.iter().find(|p| {
      return p.uuid() == uuid;
    })?;

    Some(found.clone())
  }

  pub fn piece_position(&self, uuid: Uuid) -> Option<Vector2d<i64>> {
    let dim = self.dim();

    for i in 0..dim.y {
      for j in 0..dim.x {
        if let Some(Tile::Has(current_uuid)) = self.tile(j as i64, i as i64) {
          if uuid == *current_uuid {
            return Some(Vector2d {
              x: j as i64,
              y: i as i64,
            });
          }
        }
      }
    }

    return None;
  }

  fn tile_mut(&mut self, x: i64, y: i64) -> Option<&mut Tile> {
    #[allow(unused_comparisons)]
    if !self.check_in_bounds(x, y) {
      return None;
    }

    self
      .table
      .get_mut(y as usize)
      .and_then(|row| row.get_mut(x as usize))
  }

  // Unecessary &mut self, but it is the only way of reusing tile_mut;
  pub fn tile(&self, x: i64, y: i64) -> Option<&Tile> {
    if !self.check_in_bounds(x, y) {
      return None;
    }

    self
      .table
      .get(y as usize)
      .and_then(|row| row.get(x as usize))
  }

  pub fn add_wall(&mut self, position: Vector2d<i64>) -> Result<(), BoardError> {
    let tile = self
      .tile_mut(position.x, position.y)
      .ok_or_else(|| BoardError::OutOfBound)?;

    match tile {
      Tile::Has(_) => Err(BoardError::AlreadyTaken),
      Tile::Wall => Ok(()),
      Tile::Empty => {
        (*tile) = Tile::Wall;
        Ok(())
      }
    }
  }

  pub fn add_piece(
    self: &mut Self,
    piece: Piece,
    position: Vector2d<i64>,
  ) -> Result<(), BoardError> {
    if self.piece(piece.uuid()).is_some() {
      return Err(BoardError::PieceConflict);
    };

    let target = self
      .tile_mut(position.x, position.y)
      .ok_or_else(|| BoardError::OutOfBound)?;

    match target {
      Tile::Empty => {
        *target = Tile::Has(piece.uuid());
        self.pieces.push(piece);
        Ok(())
      }
      Tile::Has(_) | Tile::Wall => Err(BoardError::AlreadyTaken),
    }
  }

  pub fn remove(&mut self, position: Vector2d<i64>) -> Result<Piece, BoardError> {
    let tile = self
      .tile_mut(position.x, position.y)
      .ok_or_else(|| BoardError::OutOfBound)?;

    match tile {
      Tile::Empty | Tile::Wall => Err(BoardError::InvalidTile),
      Tile::Has(uuid) => {
        let identification = uuid.clone();
        (*tile) = Tile::Empty;

        // remove piece from reference list
        let idx = self
          .pieces
          .iter()
          .position(|p| p.uuid() == identification)
          .ok_or_else(|| BoardError::PieceNotFound)?;

        Ok(self.pieces.remove(idx))
      }
    }
  }

  pub fn output_table(self: &Self) -> Vec<Vec<String>> {
    let dim = self.dim();
    let mut out = vec![vec![String::from("0"); dim.x]; dim.y];

    for i in 0..dim.y {
      for j in 0..dim.x {
        match self.table[i][j] {
          Tile::Empty => {}
          Tile::Wall => out[i][j] = String::from("w"),
          Tile::Has(piece_uuid) => {
            if let Some(piece) = self.piece(piece_uuid) {
              out[i][j] = format!("{}", piece.kind().id());
            }
          }
        };
      }
    }

    return out;
  }

  pub fn output(self: &Self, table: Vec<Vec<String>>) -> String {
    let mut out = String::new();

    for row in &table {
      for (idx, tile) in row.iter().enumerate() {
        // print!("{}", self.tile_id(tile));
        out = format!("{}{}", out, tile);

        if idx < row.len() {
          out = format!("{}{}", out, " ");
        }
      }

      out = format!("{}{}", out, "\n");
    }

    return out;
  }

  pub fn print(self: &Self) {
    println!("{}", self.output(self.output_table()));
  }
}
