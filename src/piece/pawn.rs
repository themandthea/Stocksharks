use crate::board::ChessBoard;
use crate::board::Board;
use crate::{Coordinate, Piece, Square,Color};
pub trait Pawn {
    fn pawn_move(&self, square: &Square) -> Vec<Square>;
}

impl Pawn for Board {
    fn pawn_move(&self, square: &Square) -> Vec<Square> {
        let mut available_moves = Vec::new();
        let piece = square.get_piece().unwrap();
        match piece {
            Piece::Pawn(Color::White) => {
                // Logic for vertical pawn movess
                let column = square.get_column().unwrap(); // Assuming column is always valid otherwise panic
                let line = square.get_line().unwrap(); // Assuming line is always valid otherwise panic
                if line != 6 {
                    // Logic for other cases
                    if let Some(square) = self.get_square(Coordinate::new(column, line + 1)) {
                        if square.available() {
                            let forward_square =
                                Square::new(Some(*piece), Coordinate::new(column, line + 1)); // 1 square mouv
                            available_moves.push(forward_square);
                        }
                    }
                    if column > 0 && line < 7 {
                        if let Some(square_left) =
                            self.get_square(Coordinate::new(column - 1, line + 1))
                        {
                            if square_left.occupied_by_oponent(&Color::White) {
                                let forward_square =
                                    Square::new(Some(*piece), Coordinate::new(column - 1, line + 1)); // Eat by left
                                available_moves.push(forward_square);
                            }
                        }
                        
                        // Capture en passant (à gauche)
                        if line == 4 { // Les pions blancs en 5ème rangée peuvent capturer en passant
                            if let Some(en_passant_coord) = &self.en_passant {
                                // Vérifier si la case en passant est à gauche du pion
                                if let Some(en_passant_column) = match en_passant_coord {
                                    Coordinate::A(r) if *r == 5 => Some(0),
                                    Coordinate::B(r) if *r == 5 => Some(1),
                                    Coordinate::C(r) if *r == 5 => Some(2),
                                    Coordinate::D(r) if *r == 5 => Some(3),
                                    Coordinate::E(r) if *r == 5 => Some(4),
                                    Coordinate::F(r) if *r == 5 => Some(5),
                                    Coordinate::G(r) if *r == 5 => Some(6),
                                    Coordinate::H(r) if *r == 5 => Some(7),
                                    _ => None,
                                } {
                                    if en_passant_column == column - 1 {
                                        // On peut capturer en passant
                                        let forward_square =
                                            Square::new(Some(*piece), Coordinate::new(column - 1, line + 1));
                                        
                                        
                                        available_moves.push(forward_square);
                                    }
                                }
                            }
                        }
                    }
                    if column < 7 && line < 7 {
                        if let Some(square_right) =
                            self.get_square(Coordinate::new(column + 1, line + 1))
                        {
                            if square_right.occupied_by_oponent(&Color::White) {
                                let forward_square =
                                    Square::new(Some(*piece), Coordinate::new(column + 1, line + 1)); // Eat by Right
                                available_moves.push(forward_square);
                            }
                        }
                        
                        // Capture en passant (à droite)
                        if line == 4 { // Les pions blancs en 5ème rangée peuvent capturer en passant
                            if let Some(en_passant_coord) = &self.en_passant {
                                // Vérifier si la case en passant est à droite du pion
                                if let Some(en_passant_column) = match en_passant_coord {
                                    Coordinate::A(r) if *r == 5 => Some(0),
                                    Coordinate::B(r) if *r == 5 => Some(1),
                                    Coordinate::C(r) if *r == 5 => Some(2),
                                    Coordinate::D(r) if *r == 5 => Some(3),
                                    Coordinate::E(r) if *r == 5 => Some(4),
                                    Coordinate::F(r) if *r == 5 => Some(5),
                                    Coordinate::G(r) if *r == 5 => Some(6),
                                    Coordinate::H(r) if *r == 5 => Some(7),
                                    _ => None,
                                } {
                                    if en_passant_column == column + 1 {
                                        // On peut capturer en passant
                                        let forward_square =
                                            Square::new(Some(*piece), Coordinate::new(column + 1, line + 1));
                                        
                                        available_moves.push(forward_square);
                                    }
                                }
                            }
                        }
                    }
                }
                match line {
                    1 => {
                        if let (Some(square1), Some(square2)) = (
                            self.get_square(Coordinate::new(column, 3)),
                            self.get_square(Coordinate::new(column, 2)),
                        ) {
                            if square1.available() && square2.available() {
                                let forward_square =
                                    Square::new(Some(*piece), Coordinate::new(column, 3)); // 2 square mouv
                                available_moves.push(forward_square);
                            }
                        }
                    }
                    4 => {
                        // La capture en passant pour les pions blancs est maintenant gérée directement dans la section générale des mouvements
                    }
                    6 => {
                        // Logic for promotion
                        if let Some(square) = self.get_square(Coordinate::new(column, line + 1)) {
                            if square.available() {
                                let bishop_promotion = Square::new(
                                    Some(Piece::Bishop(Color::White)),
                                    Coordinate::new(column, line + 1),
                                ); // 1 square mouv
                                available_moves.push(bishop_promotion);

                                let knight_promotion = Square::new(
                                    Some(Piece::Knight(Color::White)),
                                    Coordinate::new(column, line + 1),
                                ); // 1 square mouv
                                available_moves.push(knight_promotion);

                                let rook_promotion = Square::new(
                                    Some(Piece::Rook(Color::White)),
                                    Coordinate::new(column, line + 1),
                                ); // 1 square mouv
                                available_moves.push(rook_promotion);

                                let queen_promotion = Square::new(
                                    Some(Piece::Queen(Color::White)),
                                    Coordinate::new(column, line + 1),
                                ); // 1 square mouv
                                available_moves.push(queen_promotion);
                            }
                        }
                        if column > 0 {
                            if let Some(square_left) =
                                self.get_square(Coordinate::new(column - 1, line + 1))
                            {
                                if square_left.occupied_by_oponent(&Color::White) {
                                    let bishop_promotion = Square::new(
                                        Some(Piece::Bishop(Color::White)),
                                        Coordinate::new(column - 1, line + 1),
                                    ); // 1 square mouv
                                    available_moves.push(bishop_promotion);

                                    let knight_promotion = Square::new(
                                        Some(Piece::Knight(Color::White)),
                                        Coordinate::new(column - 1, line + 1),
                                    ); // 1 square mouv
                                    available_moves.push(knight_promotion);

                                    let rook_promotion = Square::new(
                                        Some(Piece::Rook(Color::White)),
                                        Coordinate::new(column - 1, line + 1),
                                    ); // 1 square mouv
                                    available_moves.push(rook_promotion);

                                    let queen_promotion = Square::new(
                                        Some(Piece::Queen(Color::White)),
                                        Coordinate::new(column - 1, line + 1),
                                    ); // 1 square mouv
                                    available_moves.push(queen_promotion);
                                }
                            }
                        }
                        if column < 7 {
                            if let Some(square_right) =
                                self.get_square(Coordinate::new(column + 1, line + 1))
                            {
                                if square_right.occupied_by_oponent(&Color::White) {
                                let bishop_promotion = Square::new(
                                    Some(Piece::Bishop(Color::White)),
                                    Coordinate::new(column + 1, line + 1),
                                ); // 1 square mouv
                                available_moves.push(bishop_promotion);

                                let knight_promotion = Square::new(
                                    Some(Piece::Knight(Color::White)),
                                    Coordinate::new(column + 1, line + 1),
                                ); // 1 square mouv
                                available_moves.push(knight_promotion);

                                let rook_promotion = Square::new(
                                    Some(Piece::Rook(Color::White)),
                                    Coordinate::new(column + 1, line + 1),
                                ); // 1 square mouv
                                available_moves.push(rook_promotion);

                                    let queen_promotion = Square::new(
                                        Some(Piece::Queen(Color::White)),
                                        Coordinate::new(column + 1, line + 1),
                                    ); // 1 square mouv
                                    available_moves.push(queen_promotion);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Piece::Pawn(Color::Black) => {
                // Logic for vertical pawn moves
                let column = square.get_column().unwrap(); // Assuming column is always valid otherwise panic
                let line = square.get_line().unwrap(); // Assuming line is always valid otherwise panic
                if line != 1 {
                    // Logic for other cases
                    if let Some(square) = self.get_square(Coordinate::new(column, line - 1)) {
                        if square.available() {
                            let forward_square =
                                Square::new(Some(*piece), Coordinate::new(column, line - 1)); // 1 square mouv
                            available_moves.push(forward_square);
                        }
                    }
                    if column > 0 && line > 0 {
                        if let Some(square_left) =
                            self.get_square(Coordinate::new(column - 1, line - 1))
                        {
                            if square_left.occupied_by_oponent(&Color::Black) {
                                let forward_square =
                                    Square::new(Some(*piece), Coordinate::new(column - 1, line - 1)); // Eat by left
                                available_moves.push(forward_square);
                            }
                        }
                        
                        // Capture en passant (à gauche)
                        if line == 3 { // Les pions noirs en 4ème rangée peuvent capturer en passant
                            if let Some(en_passant_coord) = &self.en_passant {
                                // Vérifier si la case en passant est à gauche du pion
                                if let Some(en_passant_column) = match en_passant_coord {
                                    Coordinate::A(r) if *r == 2 => Some(0),
                                    Coordinate::B(r) if *r == 2 => Some(1),
                                    Coordinate::C(r) if *r == 2 => Some(2),
                                    Coordinate::D(r) if *r == 2 => Some(3),
                                    Coordinate::E(r) if *r == 2 => Some(4),
                                    Coordinate::F(r) if *r == 2 => Some(5),
                                    Coordinate::G(r) if *r == 2 => Some(6),
                                    Coordinate::H(r) if *r == 2 => Some(7),
                                    _ => None,
                                } {
                                    if en_passant_column == column - 1 {
                                        // On peut capturer en passant
                                        let forward_square =
                                            Square::new(Some(*piece), Coordinate::new(column - 1, line - 1));
                                        
                                        available_moves.push(forward_square);
                                    }
                                }
                            }
                        }
                    }
                    if column < 7 && line > 0 {
                        if let Some(square_right) =
                            self.get_square(Coordinate::new(column + 1, line - 1))
                        {
                            if square_right.occupied_by_oponent(&Color::Black) {
                                let forward_square =
                                    Square::new(Some(*piece), Coordinate::new(column + 1, line - 1)); // Eat by Right
                                available_moves.push(forward_square);
                            }
                        }
                        
                        // Capture en passant (à droite)
                        if line == 3 { // Les pions noirs en 4ème rangée peuvent capturer en passant
                            if let Some(en_passant_coord) = &self.en_passant {
                                // Vérifier si la case en passant est à droite du pion
                                if let Some(en_passant_column) = match en_passant_coord {
                                    Coordinate::A(r) if *r == 2 => Some(0),
                                    Coordinate::B(r) if *r == 2 => Some(1),
                                    Coordinate::C(r) if *r == 2 => Some(2),
                                    Coordinate::D(r) if *r == 2 => Some(3),
                                    Coordinate::E(r) if *r == 2 => Some(4),
                                    Coordinate::F(r) if *r == 2 => Some(5),
                                    Coordinate::G(r) if *r == 2 => Some(6),
                                    Coordinate::H(r) if *r == 2 => Some(7),
                                    _ => None,
                                } {
                                    if en_passant_column == column + 1 {
                                        // On peut capturer en passant
                                        let forward_square =
                                            Square::new(Some(*piece), Coordinate::new(column + 1, line - 1));
                                        
                                        available_moves.push(forward_square);
                                    }
                                }
                            }
                        }
                    }
                }
                match line {
                    6 => {
                        if let (Some(square1), Some(square2)) = (
                            self.get_square(Coordinate::new(column, 5)),
                            self.get_square(Coordinate::new(column, 4)),
                        ) {
                            if square1.available() && square2.available() {
                                let forward_square =
                                    Square::new(Some(*piece), Coordinate::new(column, 4)); // 2 square mouv
                                available_moves.push(forward_square);
                            }
                        }
                    }
                    3 => {
                        // Capture en passant (à gauche et à droite)
                        if line == 3 { // Les pions blancs en 4ème rangée peuvent capturer en passant
                            if let Some(en_passant_coord) = &self.en_passant {
                                // Vérifier si la case en passant est à gauche ou à droite du pion
                                if let Some(en_passant_column) = match en_passant_coord {
                                    Coordinate::A(r) if *r == 2 => Some(0),
                                    Coordinate::B(r) if *r == 2 => Some(1),
                                    Coordinate::C(r) if *r == 2 => Some(2),
                                    Coordinate::D(r) if *r == 2 => Some(3),
                                    Coordinate::E(r) if *r == 2 => Some(4),
                                    Coordinate::F(r) if *r == 2 => Some(5),
                                    Coordinate::G(r) if *r == 2 => Some(6),
                                    Coordinate::H(r) if *r == 2 => Some(7),
                                    _ => None,
                                } {
                                    // Vérifier si la case en passant est à gauche du pion
                                    if en_passant_column == column - 1 {
                                        // On peut capturer en passant à gauche
                                        let forward_square =
                                            Square::new(Some(*piece), Coordinate::new(column - 1, line - 1));
  
                                        available_moves.push(forward_square);
                                    }
                                    // Vérifier si la case en passant est à droite du pion
                                    else if en_passant_column == column + 1 {
                                        // On peut capturer en passant à droite
                                        let forward_square =
                                            Square::new(Some(*piece), Coordinate::new(column + 1, line - 1));
                                         
                                        available_moves.push(forward_square);
                                    }
                                }
                            }
                        }
                    }
                    1 => {
                        // Logic for promotion
                        if let Some(square) = self.get_square(Coordinate::new(column, line + 1)) {
                            if square.available() {
                                let bishop_promotion = Square::new(
                                    Some(Piece::Bishop(Color::Black)),
                                    Coordinate::new(column, line - 1),
                                ); // 1 square mouv
                                available_moves.push(bishop_promotion);

                                let knight_promotion = Square::new(
                                    Some(Piece::Knight(Color::Black)),
                                    Coordinate::new(column, line - 1),
                                ); // 1 square mouv
                                available_moves.push(knight_promotion);

                                let rook_promotion = Square::new(
                                    Some(Piece::Rook(Color::Black)),
                                    Coordinate::new(column, line - 1),
                                ); // 1 square mouv
                                available_moves.push(rook_promotion);

                                let queen_promotion = Square::new(
                                    Some(Piece::Queen(Color::Black)),
                                    Coordinate::new(column, line - 1),
                                ); // 1 square mouv
                                available_moves.push(queen_promotion);
                            }
                        }
                        if column  > 0 && line >0 {
                            if let Some(square_left) =
                                self.get_square(Coordinate::new(column - 1, line - 1))
                            {
                                if square_left.occupied_by_oponent(&Color::Black) {
                                    let bishop_promotion = Square::new(
                                        Some(Piece::Bishop(Color::Black)),
                                        Coordinate::new(column - 1, line - 1),
                                    ); // 1 square mouv
                                    available_moves.push(bishop_promotion);

                                    let knight_promotion = Square::new(
                                        Some(Piece::Knight(Color::Black)),
                                        Coordinate::new(column - 1, line - 1),
                                    ); // 1 square mouv
                                    available_moves.push(knight_promotion);

                                    let rook_promotion = Square::new(
                                        Some(Piece::Rook(Color::Black)),
                                        Coordinate::new(column - 1, line -  1),
                                    ); // 1 square mouv
                                    available_moves.push(rook_promotion);

                                    let queen_promotion = Square::new(
                                        Some(Piece::Queen(Color::Black)),
                                        Coordinate::new(column - 1, line - 1),
                                    ); // 1 square mouv
                                    available_moves.push(queen_promotion);
                                }
                            }
                        }
                        if column + 1 <= 7 && line  >0 {
                            if let Some(square_right) =
                                self.get_square(Coordinate::new(column + 1, line -  1))
                            {
                                if square_right.occupied_by_oponent(&Color::Black) {
                                    let bishop_promotion = Square::new(
                                        Some(Piece::Bishop(Color::Black)),
                                        Coordinate::new(column + 1, line -  1),
                                    ); // 1 square mouv
                                    available_moves.push(bishop_promotion);

                                    let knight_promotion = Square::new(
                                        Some(Piece::Knight(Color::Black)),
                                        Coordinate::new(column + 1, line - 1),
                                    ); // 1 square mouv
                                    available_moves.push(knight_promotion);

                                    let rook_promotion = Square::new(
                                        Some(Piece::Rook(Color::Black)),
                                        Coordinate::new(column + 1, line - 1),
                                    ); // 1 square mouv
                                    available_moves.push(rook_promotion);

                                    let queen_promotion = Square::new(
                                        Some(Piece::Queen(Color::Black)),
                                        Coordinate::new(column + 1, line - 1),
                                    ); // 1 square mouv
                                    available_moves.push(queen_promotion);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => todo!("not the good piece"),
        }
        
        // Filtrer les mouvements qui mettent le roi en échec
        if let Some(_) = square.get_piece() {
            let from_coord = square.coordinate;
            
            available_moves.retain(|move_square| {
                self.is_move_safe(from_coord, move_square.coordinate)
            });
        }
        
        available_moves
    }
}