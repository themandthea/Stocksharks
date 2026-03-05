
use chess::{Board, Color, Piece};
use std::{cmp::Ordering, ops::Neg};
//use crate::ai::cnn::evaluate_board;


pub trait Heuristic {
    fn evaluate(&self, board: &Board) -> Evaluate;
}

#[derive(Eq, Debug, Clone, Copy)]
pub enum Evaluate {
    MateForWhite(u8),
    Eval(i32),
    MateForBlack(u8),
}
impl Evaluate {
    pub fn is_mate(&self) -> bool {
        matches!(self, Evaluate::MateForWhite(_) | Evaluate::MateForBlack(_))
    }
    pub fn to_score(&self) -> i32 {
        match self {
            Evaluate::MateForWhite(d) => 10000 - (*d as i32), // Plus le nombre de coups pour mater est petit, mieux c'est
            Evaluate::Eval(v) => *v,
            Evaluate::MateForBlack(d) => -10000 + (*d as i32), // Plus le nombre de coups pour mater est petit, mieux c'est
        }
    }
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
impl Neg for Evaluate {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Evaluate::MateForWhite(d) => Evaluate::MateForBlack(d),
            Evaluate::Eval(v) => Evaluate::Eval(-v),
            Evaluate::MateForBlack(d) => Evaluate::MateForWhite(d),
        }
    }
}

pub struct SimpleHeuristic {}
impl Heuristic for SimpleHeuristic {
    fn evaluate(&self, board: &Board) -> Evaluate {

        match board.status() {
            chess::BoardStatus::Checkmate => {
                return if board.side_to_move() == Color::White {
                    Evaluate::MateForBlack(0)
                } else {
                    Evaluate::MateForWhite(0)
                }
            }
            chess::BoardStatus::Stalemate => {
                return Evaluate::Eval(0);
            }
            _ => {}
        }
        let mut score = 0;

        let pieces = [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King];
        let white_pieces = board.color_combined(Color::White);
        let black_pieces = board.color_combined(Color::Black);

        for piece in  pieces.iter() {
            let mut value = 0; 
            let nb_pieces_white = (board.pieces(*piece) & white_pieces).count() as i32;
            let nb_pieces_black = (board.pieces(*piece) & black_pieces).count() as i32;
            match piece {
                Piece::Pawn=> value += 1,
                Piece::Knight => value += 3,
                Piece::Bishop => value += 3,
                Piece::Rook => value += 5, 
                Piece::Queen => value += 9,
                Piece::King => value += 0, // King is invaluable in terms of material
            }
            score += value * (nb_pieces_white - nb_pieces_black);
            
        }
        Evaluate::Eval(score)
    }
}

pub struct HeuristicForBlack {}
impl Heuristic for HeuristicForBlack {
    fn evaluate(&self, board: &Board) -> Evaluate {
        let simple_heuristic = SimpleHeuristic {};
        -simple_heuristic.evaluate(board)
    }
}

pub struct Ai_evaluator {
    model_path: String,
} impl Ai_evaluator {
    pub fn new(model_path: String) -> Self {
        Ai_evaluator { model_path }
    }
} 
/* 
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
    */