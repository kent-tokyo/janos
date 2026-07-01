//! `Move`: a board move or a drop-from-hand.

use crate::piece::{Piece, PieceKind};
use crate::square::Square;

/// A shogi move: either a board move or a drop
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    /// Board square this move originates from, or `None` for a drop from hand.
    pub from: Option<Square>, // None = drop
    /// Destination square.
    pub to: Square,
    /// Piece kind before promotion (or the kind being dropped).
    pub piece_kind: PieceKind, // kind before promotion (or the dropped kind)
    /// Whether the piece promotes upon completing this move.
    pub promote: bool,
}

impl Move {
    /// Construct a normal board move from `from` to `to`.
    #[inline]
    pub fn normal(from: Square, to: Square, kind: PieceKind, promote: bool) -> Self {
        Move {
            from: Some(from),
            to,
            piece_kind: kind,
            promote,
        }
    }

    /// Construct a drop of `kind` from hand onto `to`.
    #[inline]
    pub fn drop(to: Square, kind: PieceKind) -> Self {
        Move {
            from: None,
            to,
            piece_kind: kind,
            promote: false,
        }
    }

    /// True if this move is a drop from hand rather than a board move.
    #[inline]
    pub fn is_drop(self) -> bool {
        self.from.is_none()
    }
}

/// Opaque token returned by `Board::do_move`; passed to `Board::undo_move` to restore position
#[derive(Clone, Copy, Debug)]
pub struct MoveToken {
    pub(crate) from: Option<Square>,
    pub(crate) to: Square,
    pub(crate) moved: Piece,            // piece before promotion
    pub(crate) captured: Option<Piece>, // piece that was on `to`, if any
    pub(crate) promoted: bool,
    pub(crate) prev_hash: u64, // Zobrist hash before this move (restored on undo)
}
