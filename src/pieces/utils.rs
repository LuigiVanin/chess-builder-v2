use crate::{
  board::{Board, Tile},
  pieces::Piece,
  shared::{
    strategy::{Action, Move, MovementKind, Pattern},
    utils::{Direction, Vector2d},
  },
};

fn orient_movement(dir: Direction, movement: Vector2d<i64>) -> Vector2d<i64> {
  match dir {
    Direction::Up => Vector2d {
      x: movement.x,
      y: movement.y * -1,
    },
    Direction::Down => Vector2d {
      x: movement.x * -1,
      y: movement.y,
    },
    Direction::Left => Vector2d {
      x: movement.x * -1,
      y: movement.y * -1,
    },
    Direction::Right => Vector2d {
      x: movement.x,
      y: movement.y,
    },
  }
}

pub fn generate_moves(pattern: Vec<Pattern>, board: &Board, piece: &Piece) -> Vec<Move> {
  let mut moveset: Vec<Move> = vec![];

  let Some(src) = board.piece_position(piece.uuid()) else {
    return moveset;
  };

  for p in pattern {
    if let MovementKind::Normal = p.kind {
      let mut repeat = true;
      let mut current_pos = src.clone();

      while repeat {
        let d_movement = orient_movement(piece.direction, p.movement.clone());
        current_pos = current_pos + d_movement;

        if !board.check_in_bounds(current_pos.x, current_pos.y) {
          break;
        }

        let has_piece = match board.tile(current_pos.x, current_pos.y) {
          None => false,
          Some(&Tile::Has(_) | &Tile::Wall) => true,
          Some(&Tile::Empty) => false,
        };

        if has_piece {
          // TODO: Check for piece team
          if p.action.contains(&Action::Capture) {
            moveset.push(Move::new(src.clone(), current_pos.clone(), Action::Capture));
          }
          break;
        }

        if p.action.contains(&Action::Move) {
          moveset.push(Move::new(src.clone(), current_pos.clone(), Action::Move));
        }

        repeat = p.repeat;
      }
    }
  }

  moveset
}
