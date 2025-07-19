use crate::piece::bishop::Bishop;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::pawn::Pawn;
use crate::piece::queen::Queen;
use crate::piece::rook::Rook;
use crate::utils::{Coordinate, Piece, Square, Color};
use std::fmt::Display;

pub trait ChessBoard {
    fn new() -> Self;
    fn get_square(&self, coordinate: Coordinate) -> Option<&Square>;
    fn legal_moves(&self) -> Result<Vec<Board>, ()>;
    fn set_piece(&mut self, square: Coordinate, piece: Option<Piece>) -> Result<(), ()>;
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


    fn legal_moves(&self) -> Result<Vec<Board>, ()> {
        let mut count_piece = 0;
        let mut legal_boards = Vec::new();
        let color_playing = self.color_to_play;
        for square in &self.squares {
            if count_piece >= 16 {
                break; // Limite arbitraire pour éviter trop de mouvements
            }
            if let Some(piece) = square.get_piece() {
                if piece.color() != color_playing {
                    continue; // Ne considérer que les pièces du joueur actif
                }
                count_piece += 1;
                let available_moves = match piece {
                    Piece::Pawn(_) => self.pawn_move(square),
                    Piece::Knight(_) => self.knight_move(square),
                    Piece::Bishop(_) => self.bishop_move(square),
                    Piece::Rook(_) => self.rook_move(square),
                    Piece::Queen(_) => self.queen_moves(square),
                    Piece::King(_) => self.king_move(square),
                };

                for move_square in available_moves {
                    if let Some(_target_square) = self.get_square(move_square.coordinate) {
                        // Utiliser implement_move_board pour effectuer le mouvement
                        let new_board = self.implement_move_board(square.coordinate, move_square.coordinate);
                        legal_boards.push(new_board);
                    }
                }
            }
        }
        Ok(legal_boards)
        
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
    
    // Implémente un mouvement sur l'échiquier en prenant en compte les règles spéciales
    pub fn implement_move_board(&self, from: Coordinate, to: Coordinate) -> Board {
        let mut temp_board = self.clone();
        
        if let Some(from_square) = self.get_square(from) {
            if let Some(piece) = from_square.get_piece() {
                // Gestion du roque
                if let Piece::King(color) = piece {
                    let can_castle_short = match *color {
                        Color::White => self.short_castle.0,
                        Color::Black => self.short_castle.1,
                    };
                    
                    let can_castle_long = match *color {
                        Color::White => self.long_castle.0,
                        Color::Black => self.long_castle.1,
                    };
                    
                    // Extraire les coordonnées à partir de la structure Coordinate
                    let (from_file, _) = match from {
                        Coordinate::A(r) => (0, r),
                        Coordinate::B(r) => (1, r),
                        Coordinate::C(r) => (2, r),
                        Coordinate::D(r) => (3, r),
                        Coordinate::E(r) => (4, r),
                        Coordinate::F(r) => (5, r),
                        Coordinate::G(r) => (6, r),
                        Coordinate::H(r) => (7, r),
                        _ => (0, 0), // Ne devrait pas arriver
                    };
                    
                    let (to_file, _) = match to {
                        Coordinate::A(r) => (0, r),
                        Coordinate::B(r) => (1, r),
                        Coordinate::C(r) => (2, r),
                        Coordinate::D(r) => (3, r),
                        Coordinate::E(r) => (4, r),
                        Coordinate::F(r) => (5, r),
                        Coordinate::G(r) => (6, r),
                        Coordinate::H(r) => (7, r),
                        _ => (0, 0), // Ne devrait pas arriver
                    };
                    
                    // Petit roque (O-O)
                    if from_file == 4 && to_file == 6 && can_castle_short {
                        // Déplacer aussi la tour
                        let rook_from = match *color {
                            Color::White => Coordinate::H(0),
                            Color::Black => Coordinate::H(7),
                        };
                        let rook_to = match *color {
                            Color::White => Coordinate::F(0),
                            Color::Black => Coordinate::F(7),
                        };
                        
                        // Déplacer la tour
                        if let Some(rook_square) = self.get_square(rook_from) {
                            if let Some(rook_piece) = rook_square.get_piece() {
                                temp_board.set_piece(rook_to, Some(*rook_piece)).ok();
                                temp_board.set_piece(rook_from, None).ok();
                            }
                        }
                    }
                    // Grand roque (O-O-O)
                    else if from_file == 4 && to_file == 2 && can_castle_long {
                        // Déplacer aussi la tour
                        let rook_from = match *color {
                            Color::White => Coordinate::A(0),
                            Color::Black => Coordinate::A(7),
                        };
                        let rook_to = match *color {
                            Color::White => Coordinate::D(0),
                            Color::Black => Coordinate::D(7),
                        };
                        
                        // Déplacer la tour
                        if let Some(rook_square) = self.get_square(rook_from) {
                            if let Some(rook_piece) = rook_square.get_piece() {
                                temp_board.set_piece(rook_to, Some(*rook_piece)).ok();
                                temp_board.set_piece(rook_from, None).ok();
                            }
                        }
                    }
                    
                    // Désactiver les droits de roque pour le roi qui bouge (dans tous les cas)
                    match *color {
                        Color::White => {
                            temp_board.short_castle.0 = false;
                            temp_board.long_castle.0 = false;
                        },
                        Color::Black => {
                            temp_board.short_castle.1 = false;
                            temp_board.long_castle.1 = false;
                        },
                    }
                }
                
                // Gestion de l'en-passant pour les pions
                if let Piece::Pawn(color) = piece {
                    // Extraire les coordonnées à partir de la structure Coordinate
                    let (from_file, from_rank) = match from {
                        Coordinate::A(r) => (0, r),
                        Coordinate::B(r) => (1, r),
                        Coordinate::C(r) => (2, r),
                        Coordinate::D(r) => (3, r),
                        Coordinate::E(r) => (4, r),
                        Coordinate::F(r) => (5, r),
                        Coordinate::G(r) => (6, r),
                        Coordinate::H(r) => (7, r),
                        _ => (0, 0), // Ne devrait pas arriver
                    };
                    
                    let (to_file, to_rank) = match to {
                        Coordinate::A(r) => (0, r),
                        Coordinate::B(r) => (1, r),
                        Coordinate::C(r) => (2, r),
                        Coordinate::D(r) => (3, r),
                        Coordinate::E(r) => (4, r),
                        Coordinate::F(r) => (5, r),
                        Coordinate::G(r) => (6, r),
                        Coordinate::H(r) => (7, r),
                        _ => (0, 0), // Ne devrait pas arriver
                    };
                    
                    // Vérifier si c'est un déplacement de 2 cases (pour définir l'en-passant)
                    if (*color == Color::White && from_rank == 1 && to_rank == 3) ||
                       (*color == Color::Black && from_rank == 6 && to_rank == 4) {
                        // Définir la case en-passant
                        temp_board.en_passant = Some(Coordinate::new(
                            to_file,
                            if *color == Color::White { 2 } else { 5 }
                        ));
                    } else {
                        // Réinitialiser la case en-passant pour les autres mouvements
                        
                        // Vérifier si c'est une prise en passant seulement si on a une case en-passant
                        if from_file != to_file && // Déplacement diagonal
                           self.en_passant.is_some() && // Une case en-passant est disponible
                           match self.get_square(to) {
                               Some(square) => square.available(), // La case cible est vide
                               None => false,
                           } {
                            // Vérifier si la case cible correspond à la coordonnée en-passant
                            let en_passant_coord = self.en_passant.unwrap();
                            let (en_passant_file, _) = match en_passant_coord {
                                Coordinate::A(r) => (0, r),
                                Coordinate::B(r) => (1, r),
                                Coordinate::C(r) => (2, r),
                                Coordinate::D(r) => (3, r),
                                Coordinate::E(r) => (4, r),
                                Coordinate::F(r) => (5, r),
                                Coordinate::G(r) => (6, r),
                                Coordinate::H(r) => (7, r),
                                _ => (0, 0),
                            };
                            
                            if to_file == en_passant_file {
                                // C'est une prise en passant
                                let captured_pawn_coord = Coordinate::new(to_file, from_rank);
                                temp_board.set_piece(captured_pawn_coord, None).ok();
                            }
                        }
                        
                        // Réinitialiser la case en-passant après chaque mouvement qui n'est pas un double pas de pion
                        temp_board.en_passant = None;
                    }
                } else {
                    // Pour les autres pièces, réinitialiser la case en-passant
                    temp_board.en_passant = None;
                }
                
                // Gestion des droits de roque pour les tours
                if let Piece::Rook(color) = piece {
                    // Vérifier si le roque est encore possible
                    let can_castle_short = match *color {
                        Color::White => self.short_castle.0,
                        Color::Black => self.short_castle.1,
                    };
                    
                    let can_castle_long = match *color {
                        Color::White => self.long_castle.0,
                        Color::Black => self.long_castle.1,
                    };
                    
                    // Si le roque est encore possible, vérifier si c'est une tour de coin
                    if can_castle_short || can_castle_long {
                        let is_kingside = match from {
                            Coordinate::H(0) => *color == Color::White,
                            Coordinate::H(7) => *color == Color::Black,
                            _ => false,
                        };
                        
                        let is_queenside = match from {
                            Coordinate::A(0) => *color == Color::White,
                            Coordinate::A(7) => *color == Color::Black,
                            _ => false,
                        };
                        
                        if is_kingside && can_castle_short {
                            match *color {
                                Color::White => temp_board.short_castle.0 = false,
                                Color::Black => temp_board.short_castle.1 = false,
                            }
                        } else if is_queenside && can_castle_long {
                            match *color {
                                Color::White => temp_board.long_castle.0 = false,
                                Color::Black => temp_board.long_castle.1 = false,
                            }
                        }
                    }
                }
                
                // Effectuer le mouvement de base
                temp_board.set_piece(to, Some(*piece)).ok();
                temp_board.set_piece(from, None).ok();
            }
        }
        
        temp_board
    }
    
    
    // Vérifie si un mouvement est sûr pour le roi (ne le met pas en échec)
    pub fn is_move_safe(&self, from: Coordinate, to: Coordinate) -> bool {
        if let Some(from_square) = self.get_square(from) {
            if let Some(piece) = from_square.get_piece() {
                let color = piece.color();
                
                // Utiliser la fonction implement_move_board pour simuler le mouvement
                let temp_board = self.implement_move_board(from, to);
                
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
