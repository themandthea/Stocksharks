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
    fn get_square(&self, coordinate: Coordinate) -> Option<&Square>;
    fn is_king_in_check(&self, color: Color) -> bool;
    fn legal_moves(&self, piece_to_move: &Square) -> Result<Vec<Square>, ()>;
    fn set_piece(&mut self, square: Coordinate, piece: Option<Piece>) -> Result<(), ()>;
    fn update_after_move(&mut self, from: Coordinate, to: Coordinate, piece: Piece) -> Result<(), ()>;
    fn empty() -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub squares: [Square; 64],
    pub en_passant: Option<Coordinate>, // Position de la case où une capture en passant est possible
    pub long_castle: (bool, bool),      // (white, black)
    pub short_castle: (bool, bool),     // (white, black)
    pub color_to_play: Color,           // Couleur du joueur dont c'est le tour
    pub halfmove_clock: u32,            // Compteur de demi-coups pour la règle des 50 coups
    pub fullmove_number: u32,           // Nombre de coups complets
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
            en_passant: None,
            long_castle: (true, true), // Both white and black can castle
            short_castle: (true, true), // Both white and black can castle
            color_to_play: Color::White,
            halfmove_clock: 0,
            fullmove_number: 1,
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
            en_passant: None,
            long_castle: (true, true), // Both white and black can castle
            short_castle: (true, true), // Both white and black can castle
            color_to_play: Color::White,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
    fn update_after_move(&mut self, from: Coordinate, to: Coordinate, piece: Piece) -> Result<(), ()> {
        // Mise à jour de l'en passant
        self.en_passant = None; // Par défaut, aucune capture en passant possible
        
        // Si c'est un pion qui avance de deux cases, on marque la case en passant
        if let Piece::Pawn(color) = piece {
            let from_line = match from {
                Coordinate::A(rank) => rank,
                Coordinate::B(rank) => rank,
                Coordinate::C(rank) => rank,
                Coordinate::D(rank) => rank,
                Coordinate::E(rank) => rank,
                Coordinate::F(rank) => rank,
                Coordinate::G(rank) => rank,
                Coordinate::H(rank) => rank,
                Coordinate::Out => return Err(()),
            };
            
            let to_line = match to {
                Coordinate::A(rank) => rank,
                Coordinate::B(rank) => rank,
                Coordinate::C(rank) => rank,
                Coordinate::D(rank) => rank,
                Coordinate::E(rank) => rank,
                Coordinate::F(rank) => rank,
                Coordinate::G(rank) => rank,
                Coordinate::H(rank) => rank,
                Coordinate::Out => return Err(()),
            };
            
            // Si un pion blanc avance de deux cases
            if color == Color::White && from_line == 1 && to_line == 3 {
                // La case en passant est celle juste derrière le pion
                match to {
                    Coordinate::A(_) => self.en_passant = Some(Coordinate::A(2)),
                    Coordinate::B(_) => self.en_passant = Some(Coordinate::B(2)),
                    Coordinate::C(_) => self.en_passant = Some(Coordinate::C(2)),
                    Coordinate::D(_) => self.en_passant = Some(Coordinate::D(2)),
                    Coordinate::E(_) => self.en_passant = Some(Coordinate::E(2)),
                    Coordinate::F(_) => self.en_passant = Some(Coordinate::F(2)),
                    Coordinate::G(_) => self.en_passant = Some(Coordinate::G(2)),
                    Coordinate::H(_) => self.en_passant = Some(Coordinate::H(2)),
                    Coordinate::Out => {},
                }
            }
            // Si un pion noir avance de deux cases
            else if color == Color::Black && from_line == 6 && to_line == 4 {
                // La case en passant est celle juste derrière le pion
                match to {
                    Coordinate::A(_) => self.en_passant = Some(Coordinate::A(5)),
                    Coordinate::B(_) => self.en_passant = Some(Coordinate::B(5)),
                    Coordinate::C(_) => self.en_passant = Some(Coordinate::C(5)),
                    Coordinate::D(_) => self.en_passant = Some(Coordinate::D(5)),
                    Coordinate::E(_) => self.en_passant = Some(Coordinate::E(5)),
                    Coordinate::F(_) => self.en_passant = Some(Coordinate::F(5)),
                    Coordinate::G(_) => self.en_passant = Some(Coordinate::G(5)),
                    Coordinate::H(_) => self.en_passant = Some(Coordinate::H(5)),
                    Coordinate::Out => {},
                }
            }
        }
        
        // Mise à jour du halfmove_clock (règle des 50 coups)
        if let Piece::Pawn(_) = piece {
            // Si un pion a bougé, on réinitialise le compteur
            self.halfmove_clock = 0;
        } else {
            // Vérifier si c'était une capture
            if let Some(target_square) = self.get_square(to) {
                if target_square.piece.is_some() {
                    // Si une pièce a été capturée, on réinitialise le compteur
                    self.halfmove_clock = 0;
                } else {
                    // Sinon on l'incrémente
                    self.halfmove_clock += 1;
                }
            }
        }
        
        // Changement de joueur
        self.color_to_play = match self.color_to_play {
            Color::White => {
                // Incrémentation du fullmove_number après le coup des noirs
                self.fullmove_number += 1;
                Color::Black
            },
            Color::Black => Color::White,
        };
        
        Ok(())
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
                    Piece::Pawn(_) => todo!("Implement Pawn moves"),
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

// Notation FEN (Forsyth-Edwards Notation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FEN {
    // Représentation de la position en notation FEN
    pub fen_string: String,
    // Les éléments décomposés de la notation FEN
    pub position: String,     // Disposition des pièces
    pub active_color: String, // Joueur actif (w/b)
    pub castling: String,     // Possibilités de roque
    pub en_passant: String,   // Case en-passant si disponible
    pub halfmove_clock: u32,  // Compteur de demi-coups (pour la règle des 50 coups)
    pub fullmove_number: u32, // Numéro du coup complet
}

impl FEN {
    // Crée une nouvelle structure FEN à partir d'une chaîne FEN complète
    pub fn new(fen_string: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = fen_string.split_whitespace().collect();
        
        if parts.len() < 1 {
            return Err("Format FEN invalide: chaîne vide");
        }
        
        // Pour une FEN courte (juste la position), on utilise des valeurs par défaut
        let position = parts[0].to_string();
        let active_color = parts.get(1).unwrap_or(&"w").to_string();
        let castling = parts.get(2).unwrap_or(&"KQkq").to_string();
        let en_passant = parts.get(3).unwrap_or(&"-").to_string();
        let halfmove_clock = parts.get(4).unwrap_or(&"0").parse().unwrap_or(0);
        let fullmove_number = parts.get(5).unwrap_or(&"1").parse().unwrap_or(1);
        
        // Reconstruire la chaîne FEN complète si elle était partielle
        let full_fen = if parts.len() < 6 {
            format!("{} {} {} {} {} {}", 
                position, active_color, castling, en_passant, halfmove_clock, fullmove_number)
        } else {
            fen_string.to_string()
        };
        
        Ok(FEN {
            fen_string: full_fen,
            position,
            active_color,
            castling,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }
    
    // Convertit un plateau en FEN
    pub fn from_board(board: &Board) -> Self {
        let mut position = String::new();
        
        // Construction de la chaîne de position
        for rank in (0..8).rev() {
            let mut empty_count = 0;
            
            for file in 0..8 {
                let idx = (rank * 8 + file) as usize;
                let square = &board.squares[idx];
                
                if let Some(piece) = square.get_piece() {
                    if empty_count > 0 {
                        position.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    
                    let symbol = match piece {
                        Piece::Pawn(Color::White) => 'P',
                        Piece::Knight(Color::White) => 'N',
                        Piece::Bishop(Color::White) => 'B',
                        Piece::Rook(Color::White) => 'R',
                        Piece::Queen(Color::White) => 'Q',
                        Piece::King(Color::White) => 'K',
                        Piece::Pawn(Color::Black) => 'p',
                        Piece::Knight(Color::Black) => 'n',
                        Piece::Bishop(Color::Black) => 'b',
                        Piece::Rook(Color::Black) => 'r',
                        Piece::Queen(Color::Black) => 'q',
                        Piece::King(Color::Black) => 'k',
                    };
                    position.push(symbol);
                } else {
                    empty_count += 1;
                }
            }
            
            if empty_count > 0 {
                position.push_str(&empty_count.to_string());
            }
            
            // Ajouter un "/" entre les rangs sauf pour le dernier
            if rank > 0 {
                position.push('/');
            }
        }
        
        // Utiliser les valeurs du plateau
        let active_color = match board.color_to_play {
            Color::White => "w",
            Color::Black => "b",
        };
        
        // Déterminer les possibilités de roque
        let mut castling = String::new();
        if board.short_castle.0 {
            castling.push('K');
        }
        if board.long_castle.0 {
            castling.push('Q');
        }
        if board.short_castle.1 {
            castling.push('k');
        }
        if board.long_castle.1 {
            castling.push('q');
        }
        if castling.is_empty() {
            castling = "-".to_string();
        }
        
        // Convertir la case en-passant en notation algébrique
        let en_passant = match board.en_passant {
            Some(coord) => {
                let file = match coord {
                    Coordinate::A(_) => "a",
                    Coordinate::B(_) => "b",
                    Coordinate::C(_) => "c",
                    Coordinate::D(_) => "d",
                    Coordinate::E(_) => "e",
                    Coordinate::F(_) => "f",
                    Coordinate::G(_) => "g",
                    Coordinate::H(_) => "h",
                    Coordinate::Out => "-",
                };
                let rank = match coord {
                    Coordinate::A(r) | Coordinate::B(r) | Coordinate::C(r) | Coordinate::D(r) |
                    Coordinate::E(r) | Coordinate::F(r) | Coordinate::G(r) | Coordinate::H(r) => (r + 1).to_string(),
                    Coordinate::Out => "".to_string(),
                };
                if file == "-" || rank.is_empty() {
                    "-".to_string()
                } else {
                    format!("{}{}", file, rank)
                }
            },
            None => "-".to_string(),
        };
        
        // Utiliser les compteurs du plateau
        let halfmove_clock = board.halfmove_clock;
        let fullmove_number = board.fullmove_number;
        
        let fen_string = format!(
            "{} {} {} {} {} {}", 
            position, active_color, castling, en_passant, halfmove_clock, fullmove_number
        );
        
        FEN {
            fen_string,
            position,
            active_color: active_color.to_string(),
            castling,
            en_passant: en_passant.to_string(),
            halfmove_clock,
            fullmove_number,
        }
    }
    
    // Convertit une FEN en plateau
    pub fn to_board(&self) -> Result<Board, &'static str> {
        // Créer un plateau vide
        let mut board = Board::empty();
        
        // Réinitialiser les possibilités de roque
        board.short_castle = (false, false);
        board.long_castle = (false, false);
        
        // Traiter les possibilités de roque
        for c in self.castling.chars() {
            match c {
                'K' => board.short_castle.0 = true,
                'Q' => board.long_castle.0 = true,
                'k' => board.short_castle.1 = true,
                'q' => board.long_castle.1 = true,
                '-' => { /* Pas de roque possible */ },
                _ => return Err("Caractère de roque invalide"),
            }
        }
        
        // Définir le joueur actif
        board.color_to_play = match self.active_color.as_str() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Couleur du joueur actif invalide"),
        };
        
        // Traiter la case en-passant
        if self.en_passant != "-" {
            if self.en_passant.len() != 2 {
                return Err("Format de case en-passant invalide");
            }
            
            let file = match self.en_passant.chars().nth(0).unwrap() {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => return Err("Colonne de case en-passant invalide"),
            };
            
            let rank = match self.en_passant.chars().nth(1).unwrap() {
                '1' => 0,
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                '7' => 6,
                '8' => 7,
                _ => return Err("Rang de case en-passant invalide"),
            };
            
            board.en_passant = Some(Coordinate::new(file, rank));
        } else {
            board.en_passant = None;
        }
        
        // Définir les compteurs
        board.halfmove_clock = self.halfmove_clock;
        board.fullmove_number = self.fullmove_number;
        
        // Traiter la position des pièces
        let ranks: Vec<&str> = self.position.split('/').collect();
        if ranks.len() != 8 {
            return Err("Format FEN invalide: nombre de rangs incorrect");
        }
        
        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let rank_num = 7 - rank_idx as u8; // On commence par le rang 7 (haut de l'échiquier)
            let mut file: u8 = 0;
            
            for c in rank_str.chars() {
                if file >= 8 {
                    return Err("Format FEN invalide: trop de pièces sur un rang");
                }
                
                if c.is_digit(10) {
                    // Nombre de cases vides
                    let empty_count = c.to_digit(10).unwrap() as u8;
                    file += empty_count;
                } else {
                    // Pièce
                    let color = if c.is_uppercase() { Color::White } else { Color::Black };
                    let piece = match c.to_ascii_lowercase() {
                        'p' => Piece::Pawn(color),
                        'n' => Piece::Knight(color),
                        'b' => Piece::Bishop(color),
                        'r' => Piece::Rook(color),
                        'q' => Piece::Queen(color),
                        'k' => Piece::King(color),
                        _ => return Err("Caractère de pièce invalide"),
                    };
                    
                    let coordinate = Coordinate::new(file, rank_num);
                    board.set_piece(coordinate, Some(piece)).map_err(|_| "Erreur lors du placement de la pièce")?;
                    
                    file += 1;
                }
            }
            
            if file != 8 {
                return Err("Format FEN invalide: nombre de pièces incorrect sur un rang");
            }
        }
        
        Ok(board)
    }
}

// Implémentation du trait Display pour Board
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        
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
                board_string.push_str(symbol);
                board_string.push(' ');
            }
            board_string.push('\n');
        }
        
        write!(f, "{}", board_string)
    }
}

// Méthodes supplémentaires pour Board (pas dans le trait ChessBoard)
impl Board {
    // Pour la compatibilité avec le code existant
    pub fn print_board(&self) {
        print!("{}", self);
    }
    
    // Vérifie si un mouvement est sûr pour le roi (ne le met pas en échec)
    pub fn is_move_safe(&self, from: Coordinate, to: Coordinate) -> bool {
        if let Some(from_square) = self.get_square(from) {
            if let Some(piece) = from_square.get_piece() {
                let color = piece.color();
                
                // Crée une copie du plateau pour simuler le mouvement
                let mut temp_board = self.clone();
                
                // Déplace la pièce
                let _ = temp_board.set_piece(to, Some(*piece));
                let _ = temp_board.set_piece(from, None);
                
                // Vérifie si le roi est en échec après ce mouvement
                !temp_board.is_in_check(&color)
            } else {
                false
            }
        } else {
            false
        }
    }
    
    // Convertit le plateau en notation FEN
    pub fn to_fen(&self) -> String {
        FEN::from_board(self).fen_string
    }
    
    // Crée un plateau à partir d'une notation FEN
    pub fn from_fen(fen_string: &str) -> Result<Self, &'static str> {
        let fen = FEN::new(fen_string)?;
        fen.to_board()
    }
}
