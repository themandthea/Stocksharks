use std::fmt::Display;
use crate::piece::bishop::Bishop;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::pawn::Pawn;
use crate::piece::queen::Queen;
use crate::piece::rook::Rook;

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

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(piece) = &self.piece {
            write!(f, "{:?} at {:?}", piece, self.coordinate)
        } else {
            write!(f, "Empty square at {:?}", self.coordinate)
        }
    }
}

pub trait ChessBoard {
    fn new() -> Self;
    fn print_board(&self);
    fn get_square(&self, coordinate: Coordinate) -> Option<&Square>;
    fn is_king_in_check(&self, color: Color) -> bool;
    fn legal_moves(&self, piece_to_move: &Square) -> Result<Vec<Square>, ()>;
    fn set_piece(&mut self, square: Coordinate, piece: Option<Piece>) -> Result<(), ()>;
    fn set_previous_move(&mut self, from: Square, to: Square)-> Result<(), ()>;
    fn empty() -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub squares: [Square; 64],
    pub previous_move: Option<(Square, Square)>,
}
impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl ChessBoard for Board {
    fn empty() -> Self {
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
                squares[(i * 8 + j) as usize] = Square::new(None, coordinate);
            }
        }
        Board {
            squares,
            previous_move: None,
        }
    }
    fn new() -> Self {
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
            squares,
            previous_move: None,
        }
    }
    fn set_previous_move(&mut self, from: Square, to: Square)-> Result<(), ()> {
        self.previous_move = Some((from, to));
        Ok(())
    }

    fn print_board(&self) {
        for rank in (0..8).rev() {
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
    fn get_square(&self, coordinate: Coordinate) -> Option<&Square> {
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

    fn is_king_in_check(&self, _color: Color) -> bool {
        // Placeholder for check logic
        // This function should determine if the king of the given color is in check
        false
    }

    fn legal_moves(&self, piece_to_move: &Square) -> Result<Vec<Square>, ()> {
        match piece_to_move.get_piece() {
            Some(piece) => {
                let available_moves = match piece {
                    Piece::Pawn(_) => self.pawn_move(piece_to_move),
                    Piece::Knight(_) => self.knight_move(piece_to_move),
                    Piece::Bishop(_) => self.bishop_move(piece_to_move),
                    Piece::Rook(_) => self.rook_move(piece_to_move),
                    Piece::Queen(_) => self.queen_moves(piece_to_move),
                    Piece::King(_) => self.king_move(piece_to_move),
                };
                Ok(available_moves)
            }
            None => Err(()),
        }
    }

    fn set_piece(&mut self, square: Coordinate, piece: Option<Piece>) -> Result<(), ()> {
        let index = square.get_index().ok_or(())?;
        if index < self.squares.len() {
            if piece .is_none() {
                self.squares[index].set_piece(None);
            } else if let Some(p) = piece {
                if p.color() == Color::White || p.color() == Color::Black {
                    self.squares[index].set_piece(piece);
                } else {
                    return Err(());
                }
            }
            Ok(())
        } else {
            Err(())
        }
    }
}
