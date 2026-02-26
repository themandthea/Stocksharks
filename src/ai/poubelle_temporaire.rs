    pub fn alpha_beta_AI(
        &mut self,
        board: &Board,
        depth: u8,
        mut alpha: Evaluate,
        mut beta: Evaluate,
    ) -> ((Square, Square), Evaluate) {
        let color = board.side_to_move();
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
