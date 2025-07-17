use crate::chess::ChessBoard;
use crate::chess::{Board, Coordinate, Piece, Square}; // Add this import
use std::cmp::min;

pub trait Bishop {
    fn bishop_move(&self, square: &Square) -> Vec<Square>;
}

impl Bishop for Board {
    fn bishop_move(&self, square: &Square) -> Vec<Square> {
        let mut moves: Vec<Square> = vec![];
        if let Some(Piece::Bishop(color)) = square.get_piece() {
            let column = square.get_column().unwrap();
            let line = square.get_line().unwrap();

            // Diagonal moves
            for i in 1..min(column + 1, 8 - line) {
                if let Some(square_up_left) = self.get_square(Coordinate::new(column - i, line + i))
                {
                    if square_up_left.available() {
                        let square_up_left =
                            Square::new(Some(Piece::Bishop(*color)), square_up_left.coordinate);
                        moves.push(square_up_left);
                    } else if square_up_left.occupied_by_oponent(color) {
                        let square_up_left =
                            Square::new(Some(Piece::Bishop(*color)), square_up_left.coordinate);
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
                            Square::new(Some(Piece::Bishop(*color)), square_up_right.coordinate);
                        moves.push(square_up_right);
                    } else if square_up_right.occupied_by_oponent(color) {
                        let square_up_right =
                            Square::new(Some(Piece::Bishop(*color)), square_up_right.coordinate);
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
                            Square::new(Some(Piece::Bishop(*color)), square_down_left.coordinate);
                        moves.push(square_down_left);
                    } else if square_down_left.occupied_by_oponent(color) {
                        let square_down_left =
                            Square::new(Some(Piece::Bishop(*color)), square_down_left.coordinate);
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
                            Square::new(Some(Piece::Bishop(*color)), square_down_right.coordinate);
                        moves.push(square_down_right);
                    } else if square_down_right.occupied_by_oponent(color) {
                        let square_down_right =
                            Square::new(Some(Piece::Bishop(*color)), square_down_right.coordinate);
                        moves.push(square_down_right);
                        break;
                    } else {
                        break;
                    }
                }
            }
        }
        
        // Filtrer les mouvements qui mettent le roi en échec
        if let Some(piece) = square.get_piece() {
            let from_coord = square.coordinate;
            
            moves.retain(|move_square| {
                self.is_move_safe(from_coord, move_square.coordinate)
            });
        }
        
        moves
    }
}
