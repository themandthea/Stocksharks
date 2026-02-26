use chess::{Board, Color, BoardStatus,MoveGen, ChessMove};
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::ai::heuristic::{Evaluate, Heuristic, SimpleHeuristic};

pub struct Search {
    pub stack: HashMap<(u8, Board), Evaluate>,
}

impl Search {
    pub fn new() -> Self {
        Search {
            stack: HashMap::new(),
        }
    }
    pub fn get(&self, depth: u8, key: &Board) -> Option<&Evaluate> {
        self.stack.get(&(depth, *key))
    }

    pub fn insert(&mut self, depth: u8, key: Board, value: Evaluate) {
        self.stack.insert((depth, key), value);
    }

}
    pub fn alpha_beta( board: &Board, depth: u8, mut alpha: Evaluate, mut beta: Evaluate) -> (ChessMove, Evaluate)  {
        let color = board.side_to_move();
        let maximizing_player= match color{
            Color::White => true, // White is always the maximizing player
            Color::Black => false, // Black is the minimizing player
        };

        if depth == 0 {
            return (ChessMove::default(), SimpleHeuristic{}.evaluate(board))
        }
        //println!("Evaluating board at depth : {} with player : {}", depth,maximizing_player);

        let mut best_value = if maximizing_player { Evaluate::MateForBlack(0) } else { Evaluate::MateForWhite(0) };
        let mut best_move =ChessMove::default();
        let legal_move = legal_moves_ordered(board, &SimpleHeuristic{});

        match board.status() {
            BoardStatus::Checkmate => {
                return (ChessMove::default(), if maximizing_player { Evaluate::MateForBlack(0) } else { Evaluate::MateForWhite(0) });
            }
            BoardStatus::Stalemate => {
                return (ChessMove::default(), Evaluate::Eval(0));
            }
            _ => {}
        }

        for (mov, new_board) in legal_move {
            let (_, value) = alpha_beta(&new_board, depth - 1, alpha, beta);

            if maximizing_player {
                if value >= best_value {
                    best_value = value;
                    best_move =  mov;
                }
                if value > alpha {
                    alpha = value;
                }
            } else {
                if value <= best_value {
                    best_value = value;
                    best_move =  mov;
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



pub fn alpha_beta_root(board: &Board, depth: u8) -> (ChessMove, Evaluate) {
    let alpha = Evaluate::MateForBlack(255);
    let beta = Evaluate::MateForWhite(255);
    alpha_beta(board, depth, alpha, beta)
}

pub fn legal_moves_ordered<H: Heuristic>(
    board: &Board,
    heuristic: &H,
) -> Vec<(ChessMove, Board)> {
    let mut ordered_boards = Vec::new();
    let  legal_moves = MoveGen::new_legal(board);
    for mv in legal_moves {
        let new_board = board.make_move_new(mv);
        ordered_boards.push((mv, new_board));
    }
    ordered_boards.sort_by(|(mv1, a), (mv2  ,  b)| compare_boards(a, b, heuristic));
    ordered_boards
}

pub fn compare_boards<H: Heuristic>(board1: &Board, board2: &Board, heuristic: &H) -> Ordering {
    let eval1 = heuristic.evaluate(board1);
    let eval2 = heuristic.evaluate(board2);
    eval2.partial_cmp(&eval1).unwrap_or(Ordering::Equal)
}