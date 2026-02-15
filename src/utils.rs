use std::fmt::Display;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn get_index(&self) -> Option<usize> {
        match self {
            Coordinate::A(rank) => Some((rank * 8) as usize),
            Coordinate::B(rank) => Some((rank * 8 + 1) as usize),
            Coordinate::C(rank) => Some((rank * 8 + 2) as usize),
            Coordinate::D(rank) => Some((rank * 8 + 3) as usize),
            Coordinate::E(rank) => Some((rank * 8 + 4) as usize),
            Coordinate::F(rank) => Some((rank * 8 + 5) as usize),
            Coordinate::G(rank) => Some((rank * 8 + 6) as usize),
            Coordinate::H(rank) => Some((rank * 8 + 7) as usize),
            Coordinate::Out => None,
        }
    }
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
    pub fn from(coordonnee : &str) -> Self {
        if coordonnee.len() != 2 {
            return Coordinate::Out;
        }
        let file = match coordonnee.chars().nth(0).unwrap() {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 6,
            'h' | 'H' => 7,
            _ => return Coordinate::Out,
        };
        let rank = match coordonnee.chars().nth(1).unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Coordinate::Out,
        };
        Coordinate::new(file, rank)
    }
}
// Correction des noms de types pour respecter la convention Rust
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
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

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(piece) = &self.piece {
            write!(f, "{:?} at {:?}", piece, self.coordinate)
        } else {
            write!(f, "Empty square at {:?}", self.coordinate)
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Square {
            piece: None,
            coordinate: Coordinate::Out,
        }
    }
}
