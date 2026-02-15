use crate::{
    board_utils::chessboard::legal_moves_ordered,
    board_utils::chessboard::{Board, ChessBoard},
    utils::{Color, Piece, Square},
};

use crate::piece::king::King;
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::ai::cnn::evaluate_board;
pub trait Heuristic {
    fn evaluate(&self, board: &Board) -> Evaluate;
}

#[derive(Eq, Debug, Clone, Copy)]
pub enum Evaluate {
    MateForWhite(u8),
    Eval(i32),
    MateForBlack(u8),
}

impl PartialEq for Evaluate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Evaluate::MateForBlack(a), Evaluate::MateForBlack(b)) => a == b,
            (Evaluate::Eval(a), Evaluate::Eval(b)) => a == b,
            (Evaluate::MateForWhite(a), Evaluate::MateForWhite(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Evaluate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Evaluate::MateForWhite(a), Evaluate::MateForWhite(b)) => a.partial_cmp(b),
            (Evaluate::Eval(a), Evaluate::Eval(b)) => a.partial_cmp(b),
            (Evaluate::MateForWhite(_), Evaluate::Eval(_)) => Some(Ordering::Greater),
            (Evaluate::Eval(_), Evaluate::MateForWhite(_)) => Some(Ordering::Less),
            (Evaluate::MateForBlack(a), Evaluate::MateForBlack(b)) => b.partial_cmp(a),
            (Evaluate::MateForBlack(_), Evaluate::Eval(_)) => Some(Ordering::Less),
            (Evaluate::Eval(_), Evaluate::MateForBlack(_)) => Some(Ordering::Greater),
            (Evaluate::MateForWhite(_), Evaluate::MateForBlack(_)) => Some(Ordering::Greater),
            (Evaluate::MateForBlack(_), Evaluate::MateForWhite(_)) => Some(Ordering::Less),
        }
    }
}
// Permet l'utilisation directe de <= et >= avec Evaluate
impl Ord for Evaluate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
pub struct SimpleHeuristic {}
impl Heuristic for SimpleHeuristic {
    fn evaluate(&self, board: &Board) -> Evaluate {
        let mut score = 0;
        if board.is_in_check(&Color::White) {
            score += 1 // checkmate for black
        }
        if board.is_in_check(&Color::Black) {
            score -= 1 // checkmate for black
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
                score += if piece.color() == Color::White {
                    value
                } else {
                    -value
                };
            }
        }
        Evaluate::Eval(score)
    }
}

pub struct Ai_evaluator {
    model_path: String,
} impl Ai_evaluator {
    pub fn new(model_path: String) -> Self {
        Ai_evaluator { model_path }
    }
} 

impl Heuristic for Ai_evaluator {
    fn evaluate(&self, board: &Board) -> Evaluate {
        let eval = evaluate_board( &self.model_path,&board);
        let (idx, &proba) = eval.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
        let res = match idx {
            0 => Evaluate::MateForWhite((10.0*proba).round() as u8),
            1 => Evaluate::Eval((10.0*proba).round() as i32),
            2 => Evaluate::Eval(0),
            3 => Evaluate::Eval((-10.0*proba).round() as i32),
            4 => Evaluate::MateForBlack((10.0*proba).round() as u8),
            _ => Evaluate::Eval(0),
        };
        res
        
    }
}

pub struct Search {
    pub hashtable: HashMap<(u8, Board), Evaluate>,
    pub cutoff: u8,
}

impl Search {
    pub fn new() -> Self {
        Search {
            hashtable: HashMap::new(),
            cutoff: 0,
        }
    }
    pub fn get(&self, depth: u8, key: &Board) -> Option<&Evaluate> {
        self.hashtable.get(&(depth, *key))
    }

    pub fn insert(&mut self, depth: u8, key: Board, value: Evaluate) {
        self.hashtable.insert((depth, key), value);
    }

    pub fn alpha_beta(
        &mut self,
        board: &Board,
        depth: u8,
        mut alpha: Evaluate,
        mut beta: Evaluate,
    ) -> ((Square, Square), Evaluate) {
        let color = board.color_to_play();
        let maximizing_player = match color {
            Color::White => true,  // White is always the maximizing player
            Color::Black => false, // Black is the minimizing player
        };

        if let Some(value) = self.get(depth, board) {
            return ((Square::default(), Square::default()), *value);
        }

        if depth == 0 {
            let eval = Ai_evaluator::new(r"models\trained_on_puzzles.safetensors".to_string()).evaluate(board);
            return (
                (Square::default(), Square::default()),
                eval,
            );
        }
        //println!("Evaluating board at depth : {} with player : {}", depth,maximizing_player);

        let mut best_value = if maximizing_player {
            Evaluate::MateForBlack(0)
        } else {
            Evaluate::MateForWhite(0)
        };
        let mut best_move = (Square::default(), Square::default());
        let legal_move = legal_moves_ordered(board, &Ai_evaluator::new(r"models\trained_on_puzzles.safetensors".to_string()));
        //println!("Legal moves for {:?}: {:?}", color, legal_move.is_empty());
        //println!("board in check: {}", board.is_in_check(&color));
        if legal_move.is_empty() && board.is_in_check(&color) {
            println!("Checkmate");
            match color {
                Color::White => {
                    return (
                        (Square::default(), Square::default()),
                        Evaluate::MateForBlack(depth),
                    );
                } // les noirs ont maté
                Color::Black => {
                    return (
                        (Square::default(), Square::default()),
                        Evaluate::MateForWhite(depth),
                    );
                } // les blancs ont maté
            }
        }
        if legal_move.is_empty() {
            return ((Square::default(), Square::default()), Evaluate::Eval(0)); // stalemate
        }

        for (initial_pos, mov, new_board) in legal_move {
            let (_, value) = self.alpha_beta(&new_board, depth - 1, alpha, beta);

            if maximizing_player {
                if value >= best_value {
                    best_value = value;
                    best_move = (initial_pos, mov);
                }
                if value > alpha {
                    alpha = value;
                }
            } else {
                if value <= best_value {
                    best_value = value;
                    best_move = (initial_pos, mov);
                }
                if value < beta {
                    beta = value;
                }
            }

            if beta < alpha {
                break; // Alpha-Beta pruning
            }
        }
        // Store the best value in the hashtable
        self.insert(depth, *board, best_value);

        (best_move, best_value)
    }
}

/*
pub fn alpha_beta(board: &Board, depth: u8, mut alpha: Evaluate, mut beta: Evaluate) -> ((Square,Square), Evaluate) {
    let color = board.color_to_play();
    let maximizing_player= match color{
        Color::White => true, // White is always the maximizing player
        Color::Black => false, // Black is the minimizing player
    };

    if depth == 0 {
        return ((Square::default(),Square::default()), SimpleHeuristic{}.evaluate(board))
    }
    //println!("Evaluating board at depth : {} with player : {}", depth,maximizing_player);

    let mut best_value = if maximizing_player { Evaluate::MateForBlack(0) } else { Evaluate::MateForWhite(0) };
    let mut best_move =(Square::default(),Square::default());
    let legal_move = legal_moves_ordered(board, &SimpleHeuristic{});

    //println!("Legal moves for {:?}: {:?}", color, legal_move.is_empty());
    //println!("board in check: {}", board.is_in_check(&color));
    if legal_move.is_empty() && board.is_in_check(&color) {
        println!("Checkmate");
        match color {
            Color::White => return ((Square::default(),Square::default()),Evaluate::MateForBlack(depth)), // les noirs ont maté
            Color::Black => return ((Square::default(),Square::default()),Evaluate::MateForWhite(depth)), // les blancs ont maté
        }
    }
    if legal_move.is_empty() {
        return ((Square::default(),Square::default()), Evaluate::Eval(0)); // stalemate
    }

    for (initial_pos, mov, new_board) in legal_move {
        let (_, value) = alpha_beta(&new_board, depth - 1, alpha, beta);

        if maximizing_player {
            if value >= best_value {
                best_value = value;
                best_move = (initial_pos, mov);
            }
            if value > alpha {
                alpha = value;
            }
        } else {
            if value <= best_value {
                best_value = value;
                best_move = (initial_pos, mov);
            }
            if value < beta {
                beta = value;
            }
        }

        if beta < alpha {
            break; // Alpha-Beta pruning
        }
    }
    (best_move, best_value)
}
*/
pub fn alpha_beta_root(board: &Board, depth: u8) -> ((Square, Square), Evaluate) {
    let alpha = Evaluate::MateForBlack(255);
    let beta = Evaluate::MateForWhite(255);
    let mut search = Search::new();
    search.alpha_beta(board, depth, alpha, beta)
}
