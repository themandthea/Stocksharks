use crate::chess::ChessBoard;
use crate::chess::{Board, Color, Coordinate, Piece, Square}; // Add this import
pub trait Pawn {
    fn pawn_move(&self, square: Square) -> Vec<Square>;
}

impl Pawn for Board {
    fn pawn_move(&self, square: Square) -> Vec<Square> {
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
                    if let Some(square_left) =
                        self.get_square(Coordinate::new(column - 1, line + 1))
                    {
                        if square_left.occupied_by_oponent(&Color::White) {
                            let forward_square =
                                Square::new(Some(*piece), Coordinate::new(column - 1, line + 1)); // Eat by left
                            available_moves.push(forward_square);
                        }
                    }
                    if let Some(square_right) =
                        self.get_square(Coordinate::new(column + 1, line + 1))
                    {
                        if square_right.occupied_by_oponent(&Color::White) {
                            let forward_square =
                                Square::new(Some(*piece), Coordinate::new(column + 1, line + 1)); // Eat by Right
                            available_moves.push(forward_square);
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
                        // Logic for en passant left
                        if let Some((previous_square, current_square)) = self.previous_move {
                            match previous_square.get_piece() {
                                Some(Piece::Pawn(Color::Black)) if previous_square.get_line().unwrap() == 7
                                    && current_square.get_column().unwrap() == column-1
                                    && current_square.get_line().unwrap() == 5 // Previous move is a black pawn that moved 2 squares
                                    => {
                                    // En passant ifmove
                                    let en_passant_square = Square::new(Some(*piece), Coordinate::new(column-1, line+1)); // 1 square mouv
                                    available_moves.push(en_passant_square);
                                },
                                Some(Piece::Pawn(Color::Black)) if previous_square.get_line().unwrap() == 7
                                    && current_square.get_column().unwrap() == column+1
                                    && current_square.get_line().unwrap() == 5 // Previous move is a black pawn that moved 2 squares
                                    => {
                                    // En passant move
                                    let en_passant_square = Square::new(Some(*piece), Coordinate::new(column+1, line+1)); // 1 square mouv
                                    available_moves.push(en_passant_square);
                                },
                                _ => return available_moves, // Previous move is not a black pawn, return empty
                            }
                        }
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
                    if let Some(square_left) =
                        self.get_square(Coordinate::new(column - 1, line - 1))
                    {
                        if square_left.occupied_by_oponent(&Color::White) {
                            let forward_square =
                                Square::new(Some(*piece), Coordinate::new(column - 1, line - 1)); // Eat by left
                            available_moves.push(forward_square);
                        }
                    }
                    if let Some(square_right) =
                        self.get_square(Coordinate::new(column + 1, line - 1))
                    {
                        if square_right.occupied_by_oponent(&Color::White) {
                            let forward_square =
                                Square::new(Some(*piece), Coordinate::new(column + 1, line - 1)); // Eat by Right
                            available_moves.push(forward_square);
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
                        // Logic for en passant left
                        if let Some((previous_square, current_square)) = self.previous_move {
                            match previous_square.get_piece() {
                                Some(Piece::Pawn(Color::Black)) if previous_square.get_line().unwrap() == 2
                                    && current_square.get_column().unwrap() == column-1
                                    && current_square.get_line().unwrap() == 4 // Previous move is a black pawn that moved 2 squares
                                    => {
                                    // En passant ifmove
                                    let en_passant_square = Square::new(Some(*piece), Coordinate::new(column-1, line-1)); // 1 square mouv
                                    available_moves.push(en_passant_square);
                                },
                                Some(Piece::Pawn(Color::Black)) if previous_square.get_line().unwrap() == 2
                                    && current_square.get_column().unwrap() == column+1
                                    && current_square.get_line().unwrap() == 4 // Previous move is a black pawn that moved 2 squares
                                    => {
                                    // En passant move
                                    let en_passant_square = Square::new(Some(*piece), Coordinate::new(column+1, line-1)); // 1 square mouv
                                    available_moves.push(en_passant_square);
                                },
                                _ => return available_moves, // Previous move is not a black pawn, return empty
                            }
                        }
                    }
                    1 => {
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
                    _ => {}
                }
            }
            _ => todo!("not the good piece"),
        }
        available_moves
    }
}
