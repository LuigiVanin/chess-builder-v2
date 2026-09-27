use chess_lib::{
  board::{Tile, factory::BoardFactory},
  pieces::factory::PieceFactory,
  shared::{
    strategy::mask_table_with_moves,
    utils::{Direction, Vector2d},
  },
};

#[test]
fn rook_moveset() {
  println!("Hello World!");

  let mut board = BoardFactory::EmptyStandard();

  let mut rook = PieceFactory::Rook();
  rook.direction = Direction::Up;

  let pawn = PieceFactory::Pawn();

  let position = Vector2d { x: 3, y: 3 };

  if let Err(_) = board.add_piece(rook.clone(), position) {
    panic!("Should be able to place piece on the board");
  }

  if let Err(_) = board.add_piece(pawn.clone(), Vector2d { x: 3, y: 6 }) {
    panic!("Should be able to place piece on the board");
  }

  let piece = board
    .piece(rook.uuid())
    .expect("should find the piece");

  board.print();

  let moves = piece.moveset(&board);

  println!("[");
  for mov in &moves {
    println!("\t{:?}", mov);
  }
  println!("]");

  let mut table = board.output_table();

  table = mask_table_with_moves(table, &moves);

  println!("{}", board.output(table));
}

#[test]
fn queen_moveset() {
  println!("Hello World!");

  let mut board = BoardFactory::EmptyStandard();

  let mut rook = PieceFactory::Queen();
  rook.direction = Direction::Up;

  let pawn = PieceFactory::Pawn();

  let position = Vector2d { x: 3, y: 3 };

  if let Err(_) = board.add_piece(rook.clone(), position) {
    panic!("Should be able to place piece on the board");
  }

  if let Err(_) = board.add_piece(pawn.clone(), Vector2d { x: 3, y: 6 }) {
    panic!("Should be able to place piece on the board");
  }

  let piece = board
    .piece(rook.uuid())
    .expect("should find the piece");

  board.print();

  let moves = piece.moveset(&board);

  println!("[");
  for mov in &moves {
    println!("\t{:?}", mov);
  }
  println!("]");

  let mut table = board.output_table();

  table = mask_table_with_moves(table, &moves);

  println!("{}", board.output(table));
}

#[test]
fn pawn_moveset() {
  println!("Hello World!");

  let mut board = BoardFactory::EmptyStandard();

  let mut pawn = PieceFactory::Pawn();
  pawn.direction = Direction::Up;

  // let target_pawn = PieceFactory::Pawn();
  // let target_position = Vector2d { x: 4, y: 2 };

  let position = Vector2d { x: 3, y: 3 };

  let pieces_position = vec![
    Vector2d { x: 2, y: 2 },
    Vector2d { x: 2, y: 4 },
    Vector2d { x: 4, y: 4 },
    Vector2d { x: 4, y: 2 },
  ];

  if let Err(_) = board.add_piece(pawn.clone(), position) {
    panic!("Should be able to place piece on the board");
  }

  let piece = board
    .piece(pawn.uuid())
    .expect("should find the piece");

  for pos in pieces_position {
    if let Err(_) = board.add_piece(PieceFactory::Pawn(), pos) {
      panic!("Should be able to place piece on the board");
    }

    if let Some(&Tile::Has(p)) = board.tile(pos.x, pos.y) {
      board
        .piece(p)
        .expect("Should find piece");
    } else {
      panic!("Should find tile and piece");
    }
  }

  board.print();

  let moves = piece.moveset(&board);

  println!("[");
  for mov in &moves {
    println!("\t{:?}", mov);
  }
  println!("]");

  let mut table = board.output_table();

  table = mask_table_with_moves(table, &moves);

  assert_eq!(table[position.y as usize][position.x as usize], "1");
  assert_eq!(table[(position.y - 1) as usize][position.x as usize], "o");

  assert_eq!(table[2][4], "X");
  assert_eq!(table[2][2], "X");

  assert_eq!(table[4][2], "1");
  assert_eq!(table[4][4], "1");

  println!("Position: {}", table[2][4]);
  println!("{}", board.output(table));
}

#[test]
fn main() {
  println!("Hello World!");

  let mut board = BoardFactory::EmptyStandard();

  let mut pawn = PieceFactory::Pawn();
  pawn.direction = Direction::Up;

  // let target_pawn = PieceFactory::Pawn();
  // let target_position = Vector2d { x: 4, y: 2 };

  let position = Vector2d { x: 3, y: 1 };

  let pieces_position = vec![
    Vector2d { x: 2, y: 0 },
    Vector2d { x: 2, y: 2 },
    Vector2d { x: 4, y: 2 },
    Vector2d { x: 4, y: 0 },
  ];

  if let Err(_) = board.add_piece(pawn.clone(), position) {
    panic!("Should be able to place piece on the board");
  }

  let piece = board
    .piece(pawn.uuid())
    .expect("should find the piece");

  for pos in pieces_position {
    if let Err(_) = board.add_piece(PieceFactory::Pawn(), pos) {
      panic!("Should be able to place piece on the board");
    }

    if let Some(&Tile::Has(p)) = board.tile(pos.x, pos.y) {
      board
        .piece(p)
        .expect("Should find piece");
    } else {
      panic!("Should find tile and piece");
    }
  }

  board.print();

  let moves = piece.moveset(&board);

  println!("[");
  for mov in &moves {
    println!("\t{:?}", mov);
  }
  println!("]");

  let mut table = board.output_table();

  table = mask_table_with_moves(table, &moves);

  println!("Position: {}", table[1][3]);
  println!("{}", board.output(table.clone()));

  assert_eq!(table[1][3], "1");

  assert_eq!(table[0][2], "U");
  assert_eq!(table[0][3], "U");
  assert_eq!(table[0][4], "U");

  assert_eq!(table[2][2], "1");
  assert_eq!(table[2][4], "1");
}
