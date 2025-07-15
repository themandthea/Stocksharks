use crate::chess::{Board, Coordinate, Piece, Square};
use crate::chess::ChessBoard;
pub trait King {
    fn king_move(&self, square: Square) -> Vec<Square>;
}

impl King for Board {
    fn king_move(&self, square: Square) -> Vec<Square> {
        let mut moves = Vec::new();
        if let Some(Piece::King(color)) = square.get_piece() {
            // Kings moves logic

            let knight_moves: [(i8, i8); 8] = [
                (1, 1),
                (0, -1),
                (0, 1),
                (-1, -1),
                (1, -1),
                (1, 0),
                (-1, 1),
                (-1, 0),
            ];

            for &(dx, dy) in &knight_moves {
                let new_line = square.get_line().unwrap() as i8 + dx;
                let new_column = square.get_column().unwrap() as i8 + dy;
                if !(0..=7).contains(&new_line) || !(0..=7).contains(&new_column) {
                    continue; // Skip out of bounds
                }
                let new_coordinate = Coordinate::new(
                    (square.get_column().unwrap() as i8 + dy) as u8,
                    (square.get_line().unwrap() as i8 + dx) as u8,
                    );
                let new_square = Square::new(
                    Some(Piece::King(*color)),
                    new_coordinate,
                );
                if self.get_square(new_coordinate).unwrap().available() || self.get_square(new_coordinate).unwrap().occupied_by_oponent(color) {
                    moves.push(new_square);
                }
            }
        }
        moves
    }
}
