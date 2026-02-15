use crate::board_utils::chessboard::{Board, ChessBoard};
use crate::utils::{Color, Coordinate, Piece};
use std::fmt::Display;

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

        if parts.is_empty() {
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
            format!(
                "{} {} {} {} {} {}",
                position, active_color, castling, en_passant, halfmove_clock, fullmove_number
            )
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
                    Coordinate::A(r)
                    | Coordinate::B(r)
                    | Coordinate::C(r)
                    | Coordinate::D(r)
                    | Coordinate::E(r)
                    | Coordinate::F(r)
                    | Coordinate::G(r)
                    | Coordinate::H(r) => (r + 1).to_string(),
                    Coordinate::Out => "".to_string(),
                };
                if file == "-" || rank.is_empty() {
                    "-".to_string()
                } else {
                    format!("{}{}", file, rank)
                }
            }
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
                '-' => { /* Pas de roque possible */ }
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

                if c.is_ascii_digit() {
                    // Nombre de cases vides
                    let empty_count = c.to_digit(10).unwrap() as u8;
                    file += empty_count;
                } else {
                    // Pièce
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
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
                    board
                        .set_piece(coordinate, Some(piece))
                        .map_err(|_| "Erreur lors du placement de la pièce")?;

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
