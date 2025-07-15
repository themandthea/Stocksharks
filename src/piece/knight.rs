use crate::chess::{Board, Coordinate, Piece, Square};
use crate::chess::ChessBoard;
pub trait Knight {
    fn knight_move(&self, square: Square) -> Vec<Square>;
}

impl Knight for Board {
    fn knight_move(&self, square: Square) -> Vec<Square> {
        if let Some(Piece::Knight(color)) = square.get_piece() {
            // Knight moves logic
            let mut moves = Vec::new();
            let knight_moves: [(i8, i8); 8] = [
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
            ];

            for &(dx, dy) in &knight_moves {
                let new_line = square.get_line().unwrap() as i8 + dx;
                let new_column = square.get_column().unwrap() as i8 + dy;
                if !(0..=7).contains(&new_line) || !(0..=7).contains(&new_column) {
                    println!("out of bounds: ({}, {}) ", new_line, new_column);
                    continue; // Skip out of bounds
                }
                let new_coordinate = Coordinate::new(
                    (square.get_column().unwrap() as i8 + dy) as u8,
                    (square.get_line().unwrap() as i8 + dx) as u8,
                );
                let new_square = Square::new(
                    Some(Piece::Knight(*color)),
                    new_coordinate,
                );
                if self.get_square(new_coordinate).unwrap().available() || self.get_square(new_coordinate).unwrap().occupied_by_oponent(color) {
                    moves.push(new_square);
                }
            }

            moves
        } else {
            vec![]
        }
    }
}
