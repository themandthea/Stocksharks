use chess::{Board, MoveGen,ChessMove};
use chess::BoardStatus;
use crate::ai::heuristic::{self, Evaluate, SimpleHeuristic};

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
        if self.board.side_to_move() == chess::Color::White {
            return alpha_beta(&self.board, depth, i32::MIN+1, i32::MAX-1, h);
        }
        else {
            return alpha_beta(&self.board, depth, i32::MAX-1, i32::MIN+1, h);
        }
    }
}


pub fn alpha_beta<H: heuristic::Heuristic>(
    board: &Board,
    depth: u32,
    alpha: i32,
    beta: i32,
    h: &H,
) -> (Evaluate, Option<ChessMove>) {
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        println!("Evaluating board at depth : {} with player : {:?}", depth,board.side_to_move());
        return (h.evaluate(board), None);
    }

    let mut a = alpha;
    let mut best_move = None;
    for mv in MoveGen::new_legal(board) {
        println!("Depth: {}, Move: {}", depth, mv);
        let next = board.make_move_new(mv);
        let (score,_) = alpha_beta(&next, depth - 1, -beta, -a, h);
        println!("Score for move {}: {:?}", mv, score);
        let score = -score;
        if a < score.to_score() {
             best_move = Some(mv);
             a = score.to_score();
        }
        
        if a >= beta { break; }      // coup d’arrêt : coupure β
    }
    (Evaluate::Eval(a),best_move)
}