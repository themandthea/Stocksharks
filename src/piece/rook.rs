use crate::chess::ChessBoard;
use crate::chess::{Board, Coordinate, Piece, Square}; // Add this import

pub trait Rook {
    fn rook_move(&self, square: &Square) -> Vec<Square>;
}

impl Rook for Board {
    fn rook_move(&self, square: &Square) -> Vec<Square> {
        let mut available_moves = Vec::new();
        if let Some(Piece::Rook(color)) = square.get_piece() {
            // Check horizontal moves
            for line in (0..square.get_line().unwrap()).rev() {
                let column = square.get_column().unwrap();
                if let Some(square) = self.get_square(Coordinate::new(column, line )) {
                    if square.available() {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column, line ),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column, line ),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                        break;
                    } else {
                        break;
                    }
                }
            }
            for line in square.get_line().unwrap()..7 {
                let column = square.get_column().unwrap();
                if let Some(square) = self.get_square(Coordinate::new(column, line + 1)) {
                    if square.available() {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column, line + 1),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column, line + 1),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                        break;
                    } else {
                        break;
                    }
                }
            }

            // Check vertical moves
            for column in square.get_column().unwrap()..7 {
                let line = square.get_line().unwrap();
                if let Some(square) = self.get_square(Coordinate::new(column + 1, line)) {
                    if square.available() {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column + 1, line),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column + 1, line),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                        break;
                    } else {
                        break;
                    }
                }
            }
            for column in (0..square.get_column().unwrap()).rev(){
                let line = square.get_line().unwrap();
                if let Some(square) = self.get_square(Coordinate::new(column , line)) {
                    if square.available() {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column , line),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square = Square::new(
                            Some(Piece::Rook(*color)),
                            Coordinate::new(column , line),
                        ); // 1 square mouv
                        available_moves.push(forward_square);
                        break;
                    } else {
                        break;
                    }
                }
            }
        }
        available_moves
    }
}
