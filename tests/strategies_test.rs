use chess_lib::{
  board::factory::BoardFactory,
  pieces::factory::PieceFactory,
  shared::{
    strategy::Action,
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

  for mov in &moves {
    table[mov.target().y as usize][mov.target().x as usize] = match mov.action() {
      Action::Move => String::from("o"),
      Action::Capture => String::from("X"),
    }
  }

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

  for mov in &moves {
    table[mov.target().y as usize][mov.target().x as usize] = match mov.action() {
      Action::Move => String::from("x"),
      Action::Capture => String::from("K"),
    }
  }

  println!("{}", board.output(table));
}

#[test]
fn pawn_main() {
  println!("Hello World!");

  let mut board = BoardFactory::EmptyStandard();

  let mut pawn = PieceFactory::Pawn();
  pawn.direction = Direction::Up;

  let target_pawn = PieceFactory::Pawn();

  let position = Vector2d { x: 3, y: 3 };

  if let Err(_) = board.add_piece(pawn.clone(), position) {
    panic!("Should be able to place piece on the board");
  }

  if let Err(_) = board.add_piece(target_pawn.clone(), Vector2d { x: 4, y: 2 }) {
    panic!("Should be able to place piece on the board");
  }

  let piece = board
    .piece(pawn.uuid())
    .expect("should find the piece");

  board.print();

  let moves = piece.moveset(&board);

  println!("[");
  for mov in &moves {
    println!("\t{:?}", mov);
  }
  println!("]");

  let mut table = board.output_table();

  for mov in &moves {
    table[mov.target().y as usize][mov.target().x as usize] = match mov.action() {
      Action::Move => String::from("o"),
      Action::Capture => String::from("X"),
    }
  }

  println!("{}", board.output(table));
}

fn main() {
  println!("Hello World!");

  let mut board = BoardFactory::EmptyStandard();

  let mut pawn = PieceFactory::Pawn();
  pawn.direction = Direction::Up;

  let target_pawn = PieceFactory::Pawn();

  let position = Vector2d { x: 3, y: 3 };

  if let Err(_) = board.add_piece(pawn.clone(), position) {
    panic!("Should be able to place piece on the board");
  }

  if let Err(_) = board.add_piece(target_pawn.clone(), Vector2d { x: 4, y: 2 }) {
    panic!("Should be able to place piece on the board");
  }

  let piece = board
    .piece(pawn.uuid())
    .expect("should find the piece");

  board.print();

  let moves = piece.moveset(&board);

  println!("[");
  for mov in &moves {
    println!("\t{:?}", mov);
  }
  println!("]");

  let mut table = board.output_table();

  for mov in &moves {
    table[mov.target().y as usize][mov.target().x as usize] = match mov.action() {
      Action::Move => String::from("o"),
      Action::Capture => String::from("X"),
    }
  }

  println!("{}", board.output(table));
}
