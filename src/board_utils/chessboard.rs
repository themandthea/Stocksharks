use crate::ai::alpha_beta::Heuristic;
use crate::piece::bishop::Bishop;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::pawn::Pawn;
use crate::piece::queen::Queen;
use crate::piece::rook::Rook;
use crate::utils::{Coordinate, Piece, Square, Color};


pub trait ChessBoard {
    fn new() -> Self;
    fn get_square(&self, coordinate: Coordinate) -> Option<&Square>;
    fn legal_moves(&self) -> Result<Vec<(Square,Square)>, ()>;
    fn set_piece(&mut self, square: Coordinate, piece: Option<Piece>) -> Result<(), ()>;
    fn empty() -> Self;
    fn color_to_play(&self) -> Color ;
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
    fn color_to_play(&self) -> Color {
        self.color_to_play
    }
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


    fn legal_moves(&self) -> Result<Vec<(Square,Square)>, ()> {
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
                
                for moves in available_moves {
                    legal_boards.push((*square,moves));
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


// Méthodes supplémentaires pour Board (pas dans le trait ChessBoard)
impl Board {
    // Pour la compatibilité avec le code existant
    pub fn print_board(&self) {
        print!("{}", self);
    }
    
    // Implémente un mouvement sur l'échiquier en prenant en compte les règles spéciales
    pub fn implement_move_board(&self, from: Coordinate, to: Coordinate) -> Board {
        let mut temp_board = self.clone();
        temp_board.color_to_play = match self.color_to_play {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };


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



}

 // ou le chemin correct vers ton trait Heuristic

pub fn compare_boards<H: Heuristic>(
    board1: &Board,
    board2: &Board,
    heuristic: &H,
) -> std::cmp::Ordering {
    let eval1 = heuristic.evaluate(board1);
    let eval2 = heuristic.evaluate(board2);
    eval1.cmp(&eval2)
}

    pub fn legal_moves_ordered<H : Heuristic>(board  : &Board, heuristic: &H ) -> Vec<(Square,Square,Board)> {
    let mut ordered_boards = Vec::new();
    let mut legal_moves = board.legal_moves().unwrap();
    for (initial_pos, mov) in legal_moves.iter_mut() {
        let new_board = board.clone().implement_move_board(initial_pos.coordinate, mov.coordinate);
        ordered_boards.push((initial_pos.clone(), mov.clone(), new_board));
    }
    ordered_boards.sort_by(|(_, _, a), (_, _, b)| {
        compare_boards(a, b, heuristic)
    });
    ordered_boards
}


