//! `Color`: which side (Black/Sente or White/Gote) a piece or move belongs to.

/// Side to move
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    /// Sente (first player, moves up the board toward rank 1).
    Black = 0,
    /// Gote (second player, moves down the board toward rank 9).
    White = 1,
}

impl Color {
    /// The other color.
    #[inline]
    pub const fn flip(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }

    /// `0` for Black, `1` for White — used to index color-keyed tables.
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }
}
