use crate::{
    board_utils::chessboard::{Board, ChessBoard},
    utils::{Piece, Color},
};
use crate::piece::king::King;
pub trait Heuristic {
    fn evaluate(&self, board: &Board) -> i32;
}

pub struct SimpleHeuristic{

}
impl Heuristic for SimpleHeuristic {
    fn evaluate(&self, board: &Board) -> i32 {
        let mut score = 0;
        if board.is_in_check(&Color::White) {
            score +=1 // checkmate for black
        }
        if board.is_in_check(&Color::Black) {
            score -=1 // checkmate for black
        }
        
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
                score += if piece.color() == Color::White { value } else { -value };
            }
        }
        score
    }
}

pub fn alpha_beta(board: &Board, depth: usize, mut alpha: i32, mut beta: i32) -> (String, i32) {
    let color = board.color_to_play();
    let maximizing_player= match color{
        Color::White => true, // White is always the maximizing player
        Color::Black => false, // Black is the minimizing player
    };
    
    if depth == 0 {
        return ("".to_string(), SimpleHeuristic{}.evaluate(board))
    }
    println!("Evaluating board at depth : {} with player : {}", depth,maximizing_player);

    let mut best_value = if maximizing_player { i32::MIN } else { i32::MAX };
    let mut best_move ="".to_string();
    let legal_move = board.legal_moves().unwrap();
    let color_oponent = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };
    if legal_move.is_empty() && board.is_in_check(&color_oponent) {
        println!("Checkmate");
        match color {
            Color::White => return ("".to_string(),i32::MAX), // checkmate for white
            Color::Black => return ("".to_string(),i32::MIN), // checkmate for black
        }
    }
    if legal_move.is_empty() {
        return ("".to_string(), 0); // stalemate
    }
    for (initial_pos,mov) in legal_move {
        let new_board = board.clone().implement_move_board(initial_pos.coordinate,mov.coordinate);
        
        let (_, value) = alpha_beta(&new_board, depth - 1, alpha, beta);
        print!("depth : {}, Evaluating move: {} with value: {}\n",depth, board.get_uci_move(&initial_pos, &mov).unwrap(), value);

        if maximizing_player {
            if value > best_value {
                best_value = value;
                best_move = board.get_uci_move(&initial_pos, &mov).unwrap();
            }
            if value >= alpha {
                alpha = value;
            }
        } else {
            if value < best_value {
                best_value = value;
                best_move = board.get_uci_move(&initial_pos, &mov).unwrap();
            }
            if value <=  beta {
                beta = value;
            }
        }

        if beta <= alpha {
            println!("Alpha-Beta pruning at depth: {}, move: {}", depth, board.get_uci_move(&initial_pos, &mov).unwrap());
            break; // Alpha-Beta pruning
        }
    }
    (best_move, best_value)
}