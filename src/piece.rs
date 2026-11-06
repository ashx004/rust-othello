mod piece;

enum Color {
    Black = false,
    White = true,
}

// white -> true
// black -> false
pub struct Piece {
    color: bool,
}

impl Piece {
    pub fn new(color: Color) -> Piece {
        match color {
            Color::Black => {
                Self {
                    color: Color::Black,
                }
            }
            Color::White => {
                Self {
                    color: Color::White,
                }
            }
        }
    }

    pub fn flip(&self) {
        match self.color {
            Color::Black => {
                self.color = Color::White;
            }
            Color::White => {
                self.color = Color::Black;
            }
        }
    }
}