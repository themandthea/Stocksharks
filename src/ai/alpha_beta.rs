use crate::{
    board_utils::chessboard::{Board, ChessBoard},
    utils::{Piece, Color},
};
pub trait Heuristic {
    fn evaluate(&self, board: &Board, color : &Color) -> i32;
}

pub struct SimpleHeuristic{

}
impl Heuristic for SimpleHeuristic {
    fn evaluate(&self, board: &Board, color : &Color) -> i32 {
        let mut score = 0;
        for square in board.squares {
            if let Some(piece) = square.get_piece() {
                let mut value = 0;
                match piece {
                    Piece::Pawn(_) => value += 1,
                    Piece::Knight(_) => value += 3,
                    Piece::Bishop(_) => value += 3,
                    Piece::Rook(_) => value += 5,
                    Piece::Queen(_) => value += 9,
                    Piece::King(_) => value += 0, // King is invaluable in terms of material
                }
                score += if piece.color() == *color { value } else { -value };
            }
        }
        score
    }
}

pub fn alpha_beta(board: &Board, depth: usize, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> (String, i32) {
    if depth == 0 {
        let color = board.color_to_play();
        return ("".to_string(), SimpleHeuristic{}.evaluate(board, &color))
    }

    let mut best_value = if maximizing_player { i32::MIN } else { i32::MAX };
    let mut best_move ="".to_string();
    for (initial_pos,mov) in board.legal_moves().unwrap() {
        let new_board = board.clone().implement_move_board(initial_pos.coordinate,mov.coordinate);
        
        let (_, value) = alpha_beta(&new_board, depth - 1, alpha, beta, !maximizing_player);

        if maximizing_player {
            if value > best_value {
                best_value = value;
                best_move = board.get_uci_move(&initial_pos, &mov).unwrap();
            }
            if value > alpha {
                alpha = value;
            }
        } else {
            if value < best_value {
                best_value = value;
                best_move = board.get_uci_move(&initial_pos, &mov).unwrap();
            }
            if value < beta {
                beta = value;
            }
        }

        if beta <= alpha {
            break; // Alpha-Beta pruning
        }
    }
    (best_move, best_value)
}