use crate::board_utils::chessboard::{Board, ChessBoard};
use crate::utils::{Coordinate, Piece, Square};
pub trait Knight {
    fn knight_move(&self, square: &Square) -> Vec<Square>;
}

impl Knight for Board {
    fn knight_move(&self, square: &Square) -> Vec<Square> {
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
                    continue; // Skip out of bounds
                }
                let new_coordinate = Coordinate::new(
                    (square.get_column().unwrap() as i8 + dy) as u8,
                    (square.get_line().unwrap() as i8 + dx) as u8,
                );
                let new_square = Square::new(Some(Piece::Knight(*color)), new_coordinate);
                if self.get_square(new_coordinate).unwrap().available()
                    || self
                        .get_square(new_coordinate)
                        .unwrap()
                        .occupied_by_oponent(color)
                {
                    moves.push(new_square);
                }
            }

            // Filtrer les mouvements qui mettent le roi en échec
            let from_coord = square.coordinate;

            moves.retain(|move_square| self.is_move_safe(from_coord, move_square.coordinate));

            moves
        } else {
            vec![]
        }
    }
}
