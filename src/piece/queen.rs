use crate::board::ChessBoard;
use crate::board::Board;
use crate::{Coordinate, Piece, Square};
use std::cmp::min;

pub trait Queen {
    fn queen_moves(&self, square: &Square) -> Vec<Square>;
}

impl Queen for Board {
    fn queen_moves(&self, square: &Square) -> Vec<Square> {
        let mut moves: Vec<Square> = vec![];
        if let Some(Piece::Queen(color)) = square.get_piece() {
            let column = square.get_column().unwrap();
            let line = square.get_line().unwrap();

            // Diagonal moves
            for i in 1..min(column + 1, 8 - line) {
                if let Some(square_up_left) = self.get_square(Coordinate::new(column - i, line + i))
                {
                    if square_up_left.available() {
                        let square_up_left =
                            Square::new(Some(Piece::Queen(*color)), square_up_left.coordinate);
                        moves.push(square_up_left);
                    } else if square_up_left.occupied_by_oponent(color) {
                        let square_up_left =
                            Square::new(Some(Piece::Queen(*color)), square_up_left.coordinate);
                        moves.push(square_up_left);
                        break;
                    } else {
                        break;
                    }
                }
            }
            for i in 1..min(8 - column, 8 - line) {
                if let Some(square_up_right) =
                    self.get_square(Coordinate::new(column + i, line + i))
                {
                    if square_up_right.available() {
                        let square_up_right =
                            Square::new(Some(Piece::Queen(*color)), square_up_right.coordinate);
                        moves.push(square_up_right);
                    } else if square_up_right.occupied_by_oponent(color) {
                        let square_up_right =
                            Square::new(Some(Piece::Queen(*color)), square_up_right.coordinate);
                        moves.push(square_up_right);
                        break;
                    } else {
                        break;
                    }
                }
            }
            for i in 1..min(column + 1, line + 1) {
                if let Some(square_down_left) =
                    self.get_square(Coordinate::new(column - i, line - i))
                {
                    if square_down_left.available() {
                        let square_down_left =
                            Square::new(Some(Piece::Queen(*color)), square_down_left.coordinate);
                        moves.push(square_down_left);
                    } else if square_down_left.occupied_by_oponent(color) {
                        let square_down_left =
                            Square::new(Some(Piece::Queen(*color)), square_down_left.coordinate);
                        moves.push(square_down_left);
                        break;
                    } else {
                        break;
                    }
                }
            }
            for i in 1..min(8 - column, line + 1) {
                if let Some(square_down_right) =
                    self.get_square(Coordinate::new(column + i, line - i))
                {
                    if square_down_right.available() {
                        let square_down_right =
                            Square::new(Some(Piece::Queen(*color)), square_down_right.coordinate);
                        moves.push(square_down_right);
                    } else if square_down_right.occupied_by_oponent(color) {
                        let square_down_right =
                            Square::new(Some(Piece::Queen(*color)), square_down_right.coordinate);
                        moves.push(square_down_right);
                        break;
                    } else {
                        break;
                    }
                }
            }

            let piece = square.get_piece().unwrap();
            // Check horizontal moves
            for line in (0..square.get_line().unwrap()).rev() {
                let column = square.get_column().unwrap();
                if let Some(square) = self.get_square(Coordinate::new(column, line )) {
                    if square.available() {
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column, line )); // 1 square mouv
                        moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column, line )); // 1 square mouv
                        moves.push(forward_square);
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
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column, line + 1)); // 1 square mouv
                        moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column, line + 1)); // 1 square mouv
                        moves.push(forward_square);
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
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column + 1, line)); // 1 square mouv
                        moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column + 1, line)); // 1 square mouv
                        moves.push(forward_square);
                        break;
                    } else {
                        break;
                    }
                }
            }
            for column in (0..square.get_column().unwrap()).rev() {
                let line = square.get_line().unwrap();
                if let Some(square) = self.get_square(Coordinate::new(column , line)) {
                    if square.available() {
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column , line)); // 1 square mouv
                        moves.push(forward_square);
                    } else if square.occupied_by_oponent(color) {
                        let forward_square =
                            Square::new(Some(*piece), Coordinate::new(column, line)); // 1 square mouv
                        moves.push(forward_square);
                        break;
                    } else {
                        break;
                    }
                }
            }
        }

        // Filtrer les mouvements qui mettent le roi en échec
        if let Some(_) = square.get_piece() {
            let from_coord = square.coordinate;
            
            // Filtrer les mouvements qui mettent le roi en échec
            moves.retain(|move_square| {
                self.is_move_safe(from_coord, move_square.coordinate)
            });
        }

        moves
    }
}
