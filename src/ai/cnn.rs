use dfdx::prelude::*;
use crate::{
    board_utils::chessboard::{Board, ChessBoard},
    board_utils::chessboard::legal_moves_ordered,
    utils::{Piece, Color, Square},
};

use std::collections::HashMap;
use crate::piece::king::King;
use std::cmp::Ordering;

type Model = (Linear<64, 1>, ReLU, Linear<64, 3>);
pub struct ChessCNN
{
    model: Model,
    board: Board,
    dev: Cpu,
}


impl ChessCNN {
    pub fn new() -> Self {
        let dev: Cpu = Default::default();
        
        Self { (Linear<64, 1>, ReLU, Linear<64, 3>), board: ChessBoard::new(), dev }
    }

    pub fn evaluate_board(&self, b: &Board) -> f32 {
        let input = self.board_to_tensor(b);
        let output = self.model.forward(input);
        output[[0, 0]]
    }

    fn board_to_tensor(&self, board: &Board) -> Tensor2D<1, 64> {
        let mut tensor = Tensor2D::<1, 64>::zeros();
        for (i, square) in board.squares.iter().enumerate() {
            tensor[[0, i]] = match square.get_piece() {
                Some(piece) => match piece {
                    Piece::Pawn(color) => if *color == Color::White { 1.0 } else { -1.0 },
                    Piece::Knight(color) => if *color == Color::White { 2.0 } else { -2.0 },
                    Piece::Bishop(color) => if *color == Color::White { 3.0 } else { -3.0 },
                    Piece::Rook(color) => if *color == Color::White { 4.0 } else { -4.0 },
                    Piece::Queen(color) => if *color == Color::White { 5.0 } else { -5.0 },
                    Piece::King(color) => if *color == Color::White { 6.0 } else { -6.0 },
                },
                None => 0.0,
            };
        }
        tensor
    }
}

