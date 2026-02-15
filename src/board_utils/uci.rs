use crate::board_utils::chessboard::{Board, ChessBoard};

use crate::board_utils::fen::FEN;
use crate::piece::bishop::Bishop;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::pawn::Pawn;
use crate::piece::queen::Queen;
use crate::piece::rook::Rook;
use crate::utils::{Color, Coordinate, Piece, Square};

impl Board {
    // Convertit le plateau en notation FEN
    pub fn to_fen(&self) -> String {
        FEN::from_board(self).fen_string
    }

    // Crée un plateau à partir d'une notation FEN
    pub fn from_fen(fen_string: &str) -> Result<Self, &'static str> {
        let fen = FEN::new(fen_string)?;
        fen.to_board()
    }

    // Convertit une Coordinate en notation UCI (format string)
    fn coordinate_to_uci(&self, coord: Coordinate) -> String {
        let (file_char, rank) = match coord {
            Coordinate::A(r) => ('a', r),
            Coordinate::B(r) => ('b', r),
            Coordinate::C(r) => ('c', r),
            Coordinate::D(r) => ('d', r),
            Coordinate::E(r) => ('e', r),
            Coordinate::F(r) => ('f', r),
            Coordinate::G(r) => ('g', r),
            Coordinate::H(r) => ('h', r),
            Coordinate::Out => return String::from(""), // Cas impossible
        };

        format!("{}{}", file_char, rank + 1) // +1 car les rangs sont 0-indexés en interne mais 1-indexés dans la notation UCI
    }

    // Génère la notation UCI à partir de deux squares
    pub fn get_uci_move(&self, from_square: &Square, to_square: &Square) -> Option<String> {
        if let Some(piece) = from_square.get_piece() {
            // Vérifier si c'est une promotion de pion
            if let Piece::Pawn(color) = piece {
                let is_promotion = match (color, to_square.coordinate) {
                    // Pion blanc atteignant la 8ème rangée
                    (
                        Color::White,
                        Coordinate::A(7)
                        | Coordinate::B(7)
                        | Coordinate::C(7)
                        | Coordinate::D(7)
                        | Coordinate::E(7)
                        | Coordinate::F(7)
                        | Coordinate::G(7)
                        | Coordinate::H(7),
                    ) => true,
                    // Pion noir atteignant la 1ère rangée
                    (
                        Color::Black,
                        Coordinate::A(0)
                        | Coordinate::B(0)
                        | Coordinate::C(0)
                        | Coordinate::D(0)
                        | Coordinate::E(0)
                        | Coordinate::F(0)
                        | Coordinate::G(0)
                        | Coordinate::H(0),
                    ) => true,
                    _ => false,
                };

                if is_promotion {
                    let mut uci = format!(
                        "{}{}",
                        self.coordinate_to_uci(from_square.coordinate),
                        self.coordinate_to_uci(to_square.coordinate)
                    );
                    if let Some(promoted_piece) = to_square.get_piece() {
                        uci.push(match promoted_piece {
                            Piece::Queen(_) => 'q',
                            Piece::Rook(_) => 'r',
                            Piece::Bishop(_) => 'b',
                            Piece::Knight(_) => 'n',
                            _ => 'q', // Par défaut, promouvoir en dame
                        });
                    } else {
                        uci.push('q'); // Par défaut, promouvoir en dame
                    }
                    return Some(uci);
                }
            }

            // Coup normal
            Some(format!(
                "{}{}",
                self.coordinate_to_uci(from_square.coordinate),
                self.coordinate_to_uci(to_square.coordinate)
            ))
        } else {
            None
        }
    }

    // Renvoie tous les coups légaux au format UCI (format: e2e4, b1c3, etc.)
    pub fn legal_moves_uci(&self) -> Vec<String> {
        let mut uci_moves = Vec::new();
        let mut count_piece = 0;
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
                        // Créer la représentation UCI du coup
                        let from_uci = self.coordinate_to_uci(square.coordinate);
                        let to_uci = self.coordinate_to_uci(move_square.coordinate);

                        // Cas spécial pour les promotions (dans le format UCI: e7e8q, e7e8r, etc.)
                        if let Piece::Pawn(_) = piece {
                            // Détecter si c'est une promotion (pion blanc atteignant la 8ème rangée ou pion noir la 1ère)
                            let is_promotion = match move_square.coordinate {
                                Coordinate::A(7)
                                | Coordinate::B(7)
                                | Coordinate::C(7)
                                | Coordinate::D(7)
                                | Coordinate::E(7)
                                | Coordinate::F(7)
                                | Coordinate::G(7)
                                | Coordinate::H(7) => piece.color() == Color::White,
                                Coordinate::A(0)
                                | Coordinate::B(0)
                                | Coordinate::C(0)
                                | Coordinate::D(0)
                                | Coordinate::E(0)
                                | Coordinate::F(0)
                                | Coordinate::G(0)
                                | Coordinate::H(0) => piece.color() == Color::Black,
                                _ => false,
                            };

                            if is_promotion {
                                // Pour les promotions, on retourne les 4 possibilités (q, r, b, n)
                                if let Some(promoted_piece) = move_square.get_piece() {
                                    // Si la pièce sur la case cible est spécifiée (cas de promotion dans pawn_move)
                                    let promotion_char = match promoted_piece {
                                        Piece::Queen(_) => "q",
                                        Piece::Rook(_) => "r",
                                        Piece::Bishop(_) => "b",
                                        Piece::Knight(_) => "n",
                                        _ => continue, // Ne devrait pas arriver
                                    };
                                    uci_moves
                                        .push(format!("{}{}{}", from_uci, to_uci, promotion_char));
                                } else {
                                    // Sinon on ajoute toutes les promotions possibles
                                    uci_moves.push(format!("{}{}q", from_uci, to_uci)); // Dame
                                    uci_moves.push(format!("{}{}r", from_uci, to_uci)); // Tour
                                    uci_moves.push(format!("{}{}b", from_uci, to_uci)); // Fou
                                    uci_moves.push(format!("{}{}n", from_uci, to_uci)); // Cavalier
                                }
                            } else {
                                uci_moves.push(format!("{}{}", from_uci, to_uci));
                            }
                        } else {
                            uci_moves.push(format!("{}{}", from_uci, to_uci));
                        }
                    }
                }
            }
        }

        uci_moves
    }
}
