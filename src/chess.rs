#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::Pawn(color) => *color,
            Piece::Knight(color) => *color,
            Piece::Bishop(color) => *color,
            Piece::Rook(color) => *color,
            Piece::Queen(color) => *color,
            Piece::King(color) => *color,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coordinate {
    A(u8),
    B(u8),
    C(u8),
    D(u8),
    E(u8),
    F(u8),
    G(u8),
    H(u8),
    Out,
}
impl Coordinate {
    pub fn new(file: u8, rank: u8) -> Self {
        match file {
            0 => Coordinate::A(rank),
            1 => Coordinate::B(rank),
            2 => Coordinate::C(rank),
            3 => Coordinate::D(rank),
            4 => Coordinate::E(rank),
            5 => Coordinate::F(rank),
            6 => Coordinate::G(rank),
            7 => Coordinate::H(rank),
            _ => Coordinate::Out,
        }
    }
}
// Correction des noms de types pour respecter la convention Rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub piece: Option<Piece>,
    pub coordinate: Coordinate,
}

impl Square {
    pub fn new(piece: Option<Piece>, coordinate: Coordinate) -> Self {
        Square { piece, coordinate }
    }
    pub fn get_piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }
    pub fn set_piece(&mut self, piece: Option<Piece>) {
        self.piece = piece;
    }
    pub fn available(&self) -> bool {
        self.piece.is_none() && self.coordinate != Coordinate::Out
    }
    pub fn occupied_by_oponent(&self, color_piece: &Color) -> bool {
        if self.piece.is_none() {
            return false;
        }
        match self.piece.unwrap().color() {
            Color::White => color_piece == &Color::Black,
            Color::Black => color_piece == &Color::White,
        }
    }
    pub fn coordinate(&self) -> Coordinate {
        self.coordinate
    }
    pub fn get_column(&self) -> Option<u8> {
        match self.coordinate {
            Coordinate::A(_) => Some(0),
            Coordinate::B(_) => Some(1),
            Coordinate::C(_) => Some(2),
            Coordinate::D(_) => Some(3),
            Coordinate::E(_) => Some(4),
            Coordinate::F(_) => Some(5),
            Coordinate::G(_) => Some(6),
            Coordinate::H(_) => Some(7),
            Coordinate::Out => None,
        }
    }
    pub fn get_line(&self) -> Option<u8> {
        match self.coordinate {
            Coordinate::A(rank) => Some(rank),
            Coordinate::B(rank) => Some(rank),
            Coordinate::C(rank) => Some(rank),
            Coordinate::D(rank) => Some(rank),
            Coordinate::E(rank) => Some(rank),
            Coordinate::F(rank) => Some(rank),
            Coordinate::G(rank) => Some(rank),
            Coordinate::H(rank) => Some(rank),
            Coordinate::Out => None,
        }
    }
}

pub struct Board {
    pub squares: [Square; 64],
    pub previous_move: Option<(Square, Square)>,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [Square {
            piece: None,
            coordinate: Coordinate::A(1),
        }; 64];
        for i in 0..8 {
            for j in 0..8 {
                let coordinate = match j {
                    0 => Coordinate::A(i),
                    1 => Coordinate::B(i),
                    2 => Coordinate::C(i),
                    3 => Coordinate::D(i),
                    4 => Coordinate::E(i),
                    5 => Coordinate::F(i),
                    6 => Coordinate::G(i),
                    7 => Coordinate::H(i),
                    _ => panic!("Invalid file"),
                };
                let mut piece = match coordinate {
                    Coordinate::A(0) => Some(Piece::Rook(Color::White)),
                    Coordinate::B(0) => Some(Piece::Knight(Color::White)),
                    Coordinate::C(0) => Some(Piece::Bishop(Color::White)),
                    Coordinate::D(0) => Some(Piece::Queen(Color::White)),
                    Coordinate::E(0) => Some(Piece::King(Color::White)),
                    Coordinate::F(0) => Some(Piece::Bishop(Color::White)),
                    Coordinate::G(0) => Some(Piece::Knight(Color::White)),
                    Coordinate::H(0) => Some(Piece::Rook(Color::White)),
                    Coordinate::A(7) => Some(Piece::Rook(Color::Black)),
                    Coordinate::B(7) => Some(Piece::Knight(Color::Black)),
                    Coordinate::C(7) => Some(Piece::Bishop(Color::Black)),
                    Coordinate::D(7) => Some(Piece::Queen(Color::Black)),
                    Coordinate::E(7) => Some(Piece::King(Color::Black)),
                    Coordinate::F(7) => Some(Piece::Bishop(Color::Black)),
                    Coordinate::G(7) => Some(Piece::Knight(Color::Black)),
                    Coordinate::H(7) => Some(Piece::Rook(Color::Black)),
                    _ => None,
                };
                match i {
                    1 => piece = Some(Piece::Pawn(Color::White)),
                    6 => piece = Some(Piece::Pawn(Color::Black)),
                    _ => {}
                };
                squares[(i * 8 + j) as usize] = Square::new(piece, coordinate);
            }
        }
        Board {
            squares: squares,
            previous_move: None,
        }
    }

    pub fn print_board(&self) {
        for rank in 0..8 {
            for file in 0..8 {
                let idx = (rank) * 8 + (file);
                let square = &self.squares[idx];
                let symbol = match &square.piece {
                    Some(Piece::Pawn(Color::White)) => "P",
                    Some(Piece::Pawn(Color::Black)) => "p",
                    Some(Piece::Knight(Color::White)) => "N",
                    Some(Piece::Knight(Color::Black)) => "n",
                    Some(Piece::Bishop(Color::White)) => "B",
                    Some(Piece::Bishop(Color::Black)) => "b",
                    Some(Piece::Rook(Color::White)) => "R",
                    Some(Piece::Rook(Color::Black)) => "r",
                    Some(Piece::Queen(Color::White)) => "Q",
                    Some(Piece::Queen(Color::Black)) => "q",
                    Some(Piece::King(Color::White)) => "K",
                    Some(Piece::King(Color::Black)) => "k",
                    None => ".",
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
    pub fn get_square(&self, coordinate: Coordinate) -> Option<&Square> {
        let index = match coordinate {
            Coordinate::A(rank) => (rank * 8) as usize,
            Coordinate::B(rank) => (rank * 8 + 1) as usize,
            Coordinate::C(rank) => (rank * 8 + 2) as usize,
            Coordinate::D(rank) => (rank * 8 + 3) as usize,
            Coordinate::E(rank) => (rank * 8 + 4) as usize,
            Coordinate::F(rank) => (rank * 8 + 5) as usize,
            Coordinate::G(rank) => (rank * 8 + 6) as usize,
            Coordinate::H(rank) => (rank * 8 + 7) as usize,
            _ => return None,
        };
        self.squares.get(index)
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        // Placeholder for check logic
        // This function should determine if the king of the given color is in check
        false
    }

    pub fn legal_move(&self, square: Square) -> Result<Vec<Square>, ()> {
        match square.get_piece() {
            Some(piece) => {
                let mut available_moves = Vec::new();
                match piece {
                    Piece::Pawn(_) => {
                        todo!("Pawn moves not implemented yet");
                    }
                    Piece::Knight(_) => {
                        todo!("Knight moves not implemented yet");
                    }
                    Piece::Bishop(_) => {
                        todo!("Bishop moves not implemented yet");
                    }
                    Piece::Rook(_) => {
                        todo!("Rook moves not implemented yet");
                    }
                    Piece::Queen(_) => {
                        todo!("Queen moves not implemented yet");
                    }
                    Piece::King(_) => {
                        todo!("King moves not implemented yet");
                    }
                }
                // Here you would implement the logic to determine available moves for the piece
                // For now, we will just return an empty vector

                Ok(available_moves)
            }
            None => Err(()),
        }
    }
}
