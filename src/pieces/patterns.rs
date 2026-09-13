use crate::shared::{
  strategy::{Action, MovementKind, Pattern},
  utils::Vector2d,
};

pub fn pawn_pattern() -> Vec<Pattern> {
  vec![
    Pattern {
      movement: Vector2d { x: 0, y: 1 },
      kind: MovementKind::Normal,
      repeat: false,
      action: vec![Action::Move],
    },
    Pattern {
      movement: Vector2d { x: 1, y: 1 },
      kind: MovementKind::Normal,
      repeat: false,
      action: vec![Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: -1, y: 1 },
      kind: MovementKind::Normal,
      repeat: false,
      action: vec![Action::Capture],
    },
  ]
}

pub fn rook_pattern() -> Vec<Pattern> {
  vec![
    Pattern {
      movement: Vector2d { x: 0, y: 1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: 1, y: 0 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: -1, y: 0 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: 0, y: -1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
  ]
}

pub fn queen_pattern() -> Vec<Pattern> {
  vec![
    Pattern {
      movement: Vector2d { x: 0, y: 1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: 1, y: 0 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: -1, y: 0 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: 0, y: -1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: 1, y: 1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: 1, y: -1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: -1, y: 1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
    Pattern {
      movement: Vector2d { x: -1, y: -1 },
      kind: MovementKind::Normal,
      repeat: true,
      action: vec![Action::Move, Action::Capture],
    },
  ]
}
