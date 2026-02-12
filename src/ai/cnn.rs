use dfdx::prelude::*;
use crate::{
    board_utils::chessboard::{Board, ChessBoard},
    utils::{Piece, Color, Square},
};

use std::collections::HashMap;
use crate::piece::king::King;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use dfdx::optim::Sgd;

///TRAINING FUNCTION ///

enum label {
    WinForWhite(f32),
    advantageWhite(f32),
    Draw(f32),
    advantageBlack(f32),
    WinForBlack(f32),
}

// 64 squares + 1 for the color to play, 5 outputs for the evaluation (win for white, advantage for white, draw, advantage for black, win for black)
type Model = (Linear<65, 64>, ReLU, Linear<64, 5>); 

// This function converts a chess board into a tensor that can be fed into the neural network
fn board_to_tensor(board: &Board) -> Tensor2D<1, 65> {
    let dev = Cpu::default();
    let mut tensor : Tensor2D::<1, 65> = dev.sample_normal();
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
    tensor[[0, 65]] = match board.color_to_play() {
        Color::White => 1.0,
        Color::Black => -1.0,
    };
    tensor
}


fn extract_data(file_path: &str) -> Result<Vec<(Board, label)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut records = Vec::new();

    for line in contents.lines() {
        let record: Vec<String> = line.split(',').enumerate().filter_map(|(i, s)| {
            if i == 1 || i == 7 {
                Some(s.to_string())
            } else {
                None
            }
        }).collect();
        
        records.push(string_to_label(record));
    }
    Ok(records)
}

fn string_to_label(s: Vec<String>) -> (Board, label) {
    let board = Board::from_fen(&s[0]).unwrap();
    let player_color = board.color_to_play();
    let theme = &s[1];
    if theme.contains("equality"){
        return (board, label::Draw(1.0));
    }
    match player_color {
        Color::White => {
            if theme.contains("mate") || theme.contains("Mate"){
                return (board, label::WinForWhite(1.0));
            }
            
            else {
                return (board, label::advantageWhite(1.0));
            }
        },
        Color::Black => {
             if theme.contains("mate") || theme.contains("Mate"){
                return (board, label::WinForBlack(1.0));
            }
            else {
                return (board, label::advantageBlack(1.0));
            }
        },
    }
    

    
}

fn train_model(model: &mut Model, data: Vec<(Board, label)>, epochs: usize) {
    let dev = Cpu::default();    

    let mut model = dev.build_module::<Model, f32>();
    // 1. allocate gradients for the model
    let mut grads = model.alloc_grads();
    // 2. create our optimizer
    let mut opt = Sgd::new(&model, Default::default());
    // 3. trace gradients through forward pass
    for epoch in 0..epochs {
        for (board, label) in &data {
            let x = board_to_tensor(board);

            let label = label;

            let y = model.forward_mut(x.traced(grads));
            // 4. compute loss & run backpropagation

            // Create one-hot target tensor
            let mut target : Tensor2D::<1, 5> = dev.zeros();
            match label {
                label::WinForWhite(_) => target[[0, 0]] = 1.0,
                label::advantageWhite(_) => target[[0, 1]] = 1.0,
                label::Draw(_) => target[[0, 2]] = 1.0,
                label::advantageBlack(_) => target[[0, 3]] = 1.0,
                label::WinForBlack(_) => target[[0, 4]] = 1.0,
            }
            
            // Compute tensor loss (MSE)
            let loss = (y - target).square().mean();
            grads = loss.backward();
            // 5. apply gradients
            opt.update(&mut model, &grads);
        }
        
    }
    //save the model to a file
    let mut file = File::create("model.bin").unwrap();
    model.save(&mut file).unwrap();
    
}

/// POST TRAINING EVALUATION FUNCTION ///

pub fn evaluate_board(board: &Board) -> f32 {
    todo!("Implement the evaluation function using the trained model");
}