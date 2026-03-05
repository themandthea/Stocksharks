use chess::{Board, MoveGen,ChessMove};
use chess::BoardStatus;
use crate::ai::heuristic::{self, Evaluate, SimpleHeuristic, HeuristicForBlack};

#[derive(Debug, Clone, Default)]
pub struct Node {
    board: Board,
    children: Vec<Node>,
    depth: u32,
    limit: u32,
}

impl Node {
    pub fn new(board: Board, depth: u32, limit: u32) -> Self {
        Node {
            board,
            children: Vec::new(),
            depth,
            limit,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
    pub fn set_limit(&mut self, limit: u32) {
        self.limit = limit;
    }

    pub fn expand(&self) -> Node {
        let mut children = Vec::new();

        if self.depth < self.limit {
            for mv in MoveGen::new_legal(&self.board) {
                let new_board = self.board.make_move_new(mv);
                let new_node = Node::new(new_board, self.depth + 1, self.limit);

                children.push(new_node.expand());
            }
    }
        Node {
            board: self.board,
            children,
            depth: self.depth,
            limit: self.limit,
        }
    }
}
impl From<Board> for Node {
    fn from(board: Board) -> Self {
        Node::new(board, 0, 10)
    }
} impl From<Node> for Board {
    fn from(node: Node) -> Self {
        node.board
    }
} impl From<&Node> for Board {
    fn from(node: &Node) -> Self {
        node.board
    }
} impl From<&Board> for Node {
    fn from(board: &Board) -> Self {
        Node::new(*board, 0, 10)
    }
} impl From<String> for Node {
    fn from(fen: String) -> Self {
        let board = Board::from_fen(fen).unwrap();
        Node::new(board, 0, 10)
    }   
} 


impl Node {
    pub fn evaluate_path<H: heuristic::Heuristic>(
        &self, depth: u32, h: &H
    ) -> (Evaluate, Option<ChessMove>) {
        if (self.board.side_to_move() == chess::Color::White) {
            print!("let evaluate for white : ");
            alpha_beta(&self.board, depth, Evaluate::MateForBlack(1), Evaluate::MateForWhite(1), h)

        } else {
            print!("let evaluate for black : ");
            alpha_beta(&self.board, depth, Evaluate::MateForBlack(1), Evaluate::MateForWhite(1), &HeuristicForBlack{})

        }
    }
}


pub fn alpha_beta<H: heuristic::Heuristic>(
    board: &Board,
    depth: u32,
    alpha: Evaluate,
    beta: Evaluate,
    h: &H,
) -> (Evaluate, Option<ChessMove>) {
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return (h.evaluate(board), None);
    }

    let mut a = alpha;
    let mut best_move = None;
    for mv in MoveGen::new_legal(board) {
        let next = board.make_move_new(mv);
        let (mut score,_) = alpha_beta(&next, depth - 1, -beta, -a, h);
        match score {
            Evaluate::MateForWhite(d) => {
                score = Evaluate::MateForWhite(d + 1);
            }
            Evaluate::MateForBlack(d) => {
                score = Evaluate::MateForBlack(d + 1);
            }
            _ => {}
        }
        score = -score;
        if a < score {
             best_move = Some(mv);
             a = score;
        }
        
        if a >= beta { //println!("Pruning at depth {:?} with alpha {:?} beta {:?}", depth, &a, beta);
            break; }      
    }
    (a,best_move)
}