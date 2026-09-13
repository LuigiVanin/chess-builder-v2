pub enum GameError {
  Board(BoardError),
}

pub struct Game {
  board: Board,
  players: Vec<Player>,
}
