use chess_lib::{
  board::{BoardError, Tile, factory::BoardFactory},
  pieces::factory::PieceFactory,
  shared::utils::Vector2d,
};
use std::assert_matches;

#[test]
fn initilize_board() {
  let board = BoardFactory::EmptyStandard();

  let dim = board.dim();
  assert_eq!(dim.x, 8);
  assert_eq!(dim.y, 8);

  for y in 0..dim.y {
    for x in 0..dim.x {
      match board.tile(x as i64, y as i64) {
        None => panic!("Should be elegible tile"),
        Some(Tile::Has(_) | Tile::Wall) => panic!("Should be empty"),
        Some(Tile::Empty) => {
          assert!(true);
        }
      }
    }
  }
}

#[test]
fn out_of_bound_tile() {
  let mut board = BoardFactory::EmptyStandard();

  let dim = board.dim();

  board.print();

  let pawn = PieceFactory::Pawn();

  assert_matches!(
    board.add_piece(
      pawn.clone(),
      Vector2d {
        x: dim.x as i64 * 2,
        y: dim.y as i64 * 2,
      },
    ),
    Err(BoardError::OutOfBound)
  );

  assert_matches!(
    board.add_piece(
      pawn.clone(),
      Vector2d {
        x: dim.x as i64 * 0,
        y: dim.y as i64 * 2,
      },
    ),
    Err(BoardError::OutOfBound)
  );

  assert_matches!(
    board.add_piece(
      pawn.clone(),
      Vector2d {
        x: (dim.x / 2) as i64 * -1,
        y: (dim.y / 2) as i64 * -1,
      },
    ),
    Err(BoardError::OutOfBound)
  )
}

#[test]
fn add_piece_filling_board() {
  let mut board = BoardFactory::EmptyStandard();

  let mut positions: Vec<Vector2d<i64>> = vec![];
  let dim = board.dim();

  for i in 0..(dim.x * dim.y) {
    positions.push(Vector2d {
      x: (i as i64) % (dim.x as i64),
      y: (i as i64) / (dim.y as i64),
    });
  }

  for position in positions {
    let pawn = PieceFactory::Pawn();

    println!("Pawn UUID: {}", pawn.uuid());

    assert_matches!(board.add_piece(pawn.clone(), position), Ok(()));

    let tile = board.tile(position.x, position.y);

    match tile {
      None | Some(&Tile::Empty | &Tile::Wall) => panic!("Should Always be a piece"),
      Some(&Tile::Has(piece_id)) => {
        println!("Found Piece UUID: {}", piece_id);

        assert_eq!(piece_id, pawn.uuid(), "Not the same piece");
      }
    };

    let reference_piece = board.piece(pawn.uuid());

    assert_eq!(reference_piece.is_none(), false);
    assert_eq!(reference_piece.clone().unwrap().uuid(), pawn.uuid());
    assert!(reference_piece.unwrap().kind() == pawn.kind());

    let piece_position = board.piece_position(pawn.uuid());

    match piece_position {
      None => panic!("Piece not found!"),
      Some(pos) => {
        assert_eq!(pos.x, position.x, "Wrong x piece position");
        assert_eq!(pos.y, position.y, "Wrong y piece position");
      }
    }

    assert_matches!(
      board.add_piece(pawn, Vector2d { x: 3, y: 3 }),
      Err(BoardError::PieceConflict)
    );

    assert_matches!(
      board.add_piece(PieceFactory::Pawn(), position),
      Err(BoardError::AlreadyTaken)
    );
    board.print();
  }
}

#[test]
fn add_piece_invalid_position() {
  let mut board = BoardFactory::EmptyStandard();
  let dim = board.dim();

  let invalid_positions: Vec<Vector2d<i64>> = vec![
    Vector2d {
      x: dim.x as i64 * 2,
      y: dim.y as i64 * 2,
    },
    Vector2d {
      x: dim.x as i64 * 0,
      y: dim.y as i64 * 2,
    },
    Vector2d {
      x: dim.x as i64 * 2,
      y: dim.y as i64 * 0,
    },
    Vector2d {
      x: dim.x as i64 * -1,
      y: dim.y as i64 * -1,
    },
    Vector2d {
      x: dim.x as i64 * 0,
      y: dim.y as i64 * -1,
    },
    Vector2d {
      x: dim.x as i64 * -1,
      y: dim.y as i64 * 0,
    },
  ];

  for position in invalid_positions {
    assert_matches!(
      board.add_piece(PieceFactory::Pawn(), position),
      Err(BoardError::OutOfBound)
    );
  }
}

#[test]
fn standard_board() {
  let board = BoardFactory::Standard();

  board.print();

  let dim = board.dim();

  for x in 0..dim.x {
    let tile = board.tile(x as i64, 1);

    assert!(!tile.is_none());

    if let Some(Tile::Has(piece_id)) = tile {
      let piece = board.piece(*piece_id);
      assert!(!piece.is_none());

      assert_eq!(piece.unwrap().kind().code(), "pawn");
    }

    let tile = board.tile(x as i64, 6);

    assert!(!tile.is_none());

    if let Some(Tile::Has(piece_id)) = tile {
      let piece = board.piece(*piece_id);
      assert!(!piece.is_none());

      assert_eq!(piece.unwrap().kind().code(), "pawn");
    }
  }

  let pieces_code_order = [
    "rook", "knight", "bishop", "king", "queen", "bishop", "knight", "rook",
  ];

  for (idx, code) in pieces_code_order.iter().enumerate() {
    if let Some(&Tile::Has(piece_uuid)) = board.tile(idx as i64, 0) {
      let piece = board.piece(piece_uuid).unwrap();

      assert_eq!(piece.kind().code(), code.to_string());
    } else {
      panic!("Should be occupied")
    }
  }

  for (idx, code) in pieces_code_order
    .iter()
    .rev()
    .enumerate()
  {
    if let Some(&Tile::Has(piece_uuid)) = board.tile(idx as i64, 7) {
      let piece = board.piece(piece_uuid).unwrap();

      assert_eq!(piece.kind().code(), code.to_string());
    } else {
      panic!("Should be occupied")
    }
  }
}

#[test]
fn removing_all_board() {
  let mut board = BoardFactory::Standard();

  let dim = board.dim();

  for x in 0..dim.x {
    let position = Vector2d { x: x as i64, y: 1 };

    let piece = board.remove(position).unwrap();

    assert_eq!(piece.kind().code(), "pawn");
    assert!(board.piece(piece.uuid()).is_none());
    assert_matches!(board.tile(position.x, position.y), Some(&Tile::Empty));

    match board.remove(position) {
      Err(BoardError::InvalidTile) => {}
      Ok(_) | Err(_) => {
        panic!("Should be empty");
      }
    }
  }

  for x in 0..dim.x {
    let position = Vector2d { x: x as i64, y: 6 };

    let piece = board.remove(position).unwrap();

    assert_eq!(piece.kind().code(), "pawn");
    assert!(board.piece(piece.uuid()).is_none());
    assert_matches!(board.tile(position.x, position.y), Some(&Tile::Empty));

    match board.remove(position) {
      Err(BoardError::InvalidTile) => {}
      Ok(_) | Err(_) => {
        panic!("Should be empty");
      }
    }
  }

  let pieces_code_order = [
    "rook", "knight", "bishop", "king", "queen", "bishop", "knight", "rook",
  ];

  for (idx, code) in pieces_code_order.iter().enumerate() {
    let position = Vector2d {
      x: idx as i64,
      y: 0,
    };
    let piece = board.remove(position).unwrap();
    assert_eq!(piece.kind().code(), code.to_string());
    assert!(board.piece(piece.uuid()).is_none());
    assert_matches!(board.tile(position.x, position.y), Some(&Tile::Empty));

    match board.remove(position) {
      Err(BoardError::InvalidTile) => {}
      Ok(_) | Err(_) => {
        panic!("Should be empty");
      }
    }
  }

  for (idx, code) in pieces_code_order
    .iter()
    .rev()
    .enumerate()
  {
    let position = Vector2d {
      x: idx as i64,
      y: 7,
    };
    let piece = board.remove(position).unwrap();

    assert_eq!(piece.kind().code(), code.to_string());
    assert!(board.piece(piece.uuid()).is_none());
    assert_matches!(board.tile(position.x, position.y), Some(&Tile::Empty));

    match board.remove(position) {
      Err(BoardError::InvalidTile) => {}
      Ok(_) | Err(_) => {
        panic!("Should be empty");
      }
    }
  }

  board.print();
}

#[test]
fn remove_out_of_bound() {
  let mut board = BoardFactory::Standard();

  let dim = board.dim();

  match board.remove(Vector2d {
    x: (dim.x as i64) * -1,
    y: (dim.y as i64) * -1,
  }) {
    Err(BoardError::OutOfBound) => {}
    Ok(_) | Err(_) => panic!(""),
  }

  match board.remove(Vector2d {
    x: (dim.x as i64) * 2,
    y: (dim.y as i64) * 2,
  }) {
    Err(BoardError::OutOfBound) => {}
    Ok(_) | Err(_) => panic!(""),
  }
}
