use crate::{
  board::{Board, builder::BoardBuilder},
  shared::{interfaces::Builder, utils::Direction},
};

pub struct BoardFactory;

impl BoardFactory {
  #[allow(non_snake_case)]
  pub fn Standard() -> Board {
    let width = 8;
    let height = 8;
    let mut builder = BoardBuilder::new(height as usize, width as usize);

    for x in 0..width {
      builder = builder.add_pawn(Direction::Down, x, 1);
      builder = builder.add_pawn(Direction::Up, x, 6);
    }

    let top = height - 1;
    let bottom = 0;
    builder = builder
      .add_rook(Direction::Up, 0, top)
      .add_rook(Direction::Up, width - 1, top)
      .add_rook(Direction::Down, 0, bottom)
      .add_rook(Direction::Down, width - 1, bottom);

    builder = builder
      .add_knight(Direction::Up, 1, top)
      .add_knight(Direction::Up, width - 2, top)
      .add_knight(Direction::Down, 1, bottom)
      .add_knight(Direction::Down, width - 2, bottom);

    builder = builder
      .add_bishop(Direction::Up, 2, top)
      .add_bishop(Direction::Up, width - 3, top)
      .add_bishop(Direction::Down, 2, bottom)
      .add_bishop(Direction::Down, width - 3, bottom);

    builder = builder
      .add_bishop(Direction::Up, 2, top)
      .add_bishop(Direction::Up, width - 3, top)
      .add_bishop(Direction::Down, 2, bottom)
      .add_bishop(Direction::Down, width - 3, bottom);

    builder = builder
      .add_queen(Direction::Up, 3, top)
      .add_queen(Direction::Down, width - 4, bottom);

    builder = builder
      .add_king(Direction::Up, 4, top)
      .add_king(Direction::Down, width - 5, bottom);

    return builder.build();
  }

  #[allow(non_snake_case)]
  pub fn EmptyStandard() -> Board {
    let board = Board::new(8, 8);
    return board;
  }
}
