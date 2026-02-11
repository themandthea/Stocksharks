use dfdx::prelude::*;
use crate::{
    board_utils::chessboard::{Board, ChessBoard},
    board_utils::chessboard::legal_moves_ordered,
    utils::{Piece, Color, Square},
};

use std::collections::HashMap;
use crate::piece::king::King;
use std::cmp::Ordering;

///TRAINING FUNCTION ///

type Model = (Linear<64, 1>, ReLU, Linear<64, 3>);

fn board_to_tensor(board: &Board) -> Tensor2D<1, 64> {
    let dev = Cpu::default();
    let mut tensor : Tensor2D::<1, 64> = dev.sample_normal();
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

/// POST TRAINING EVALUATION FUNCTION ///

pub fn evaluate_board(board: &Board) -> f32 {
    todo!("Implement the evaluation function using the trained model");
}