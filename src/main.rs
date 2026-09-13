use chess_lib::{
  board::BoardError, board::factory::BoardFactory, pieces::factory::PieceFactory,
  shared::utils::Vector2d,
};
use uuid::Uuid;

fn main() -> Result<(), BoardError> {
  let id = Uuid::new_v4();

  println!("Hello, world! {}", id.to_string());

  let mut board = BoardFactory::Standard();
  let pawn = PieceFactory::Pawn();

  println!("{}", pawn.name());

  board.print();

  board.add_piece(pawn, Vector2d { x: 3, y: 7 })?;

  println!("\n");

  board.print();

  Ok(())
}
