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

pub fn should_promote(
  actions: &Vec<Action>,
  target_pos: Vector2d<i64>,
  piece: &Piece,
  board: &Board,
) -> bool {
  let dim = board.dim();
  if actions.contains(&Action::Promote) {
    match piece.direction {
      Direction::Down => {
        if target_pos.y == (dim.y - 1) as i64 {
          return true;
        }
      }
      Direction::Up => {
        if target_pos.y == 0 {
          return true;
        }
      }
      Direction::Right => {
        if target_pos.x == (dim.x - 1) as i64 {
          return true;
        }
      }
      Direction::Left => {
        if target_pos.x == 0 {
          return true;
        }
      }
    }
  };

  false
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
            let mut action = vec![Action::Capture];

            if should_promote(&p.action, current_pos, piece, board) {
              action.push(Action::Promote);
            }

            moveset.push(Move::new(src.clone(), current_pos.clone(), action));
          }
          break;
        }

        if p.action.contains(&Action::Move) {
          let mut action = vec![Action::Move];

          if should_promote(&p.action, current_pos, piece, board) {
            action.push(Action::Promote);
          }

          moveset.push(Move::new(src.clone(), current_pos.clone(), action));
        }

        repeat = p.repeat;
      }
    }
  }

  moveset
}
