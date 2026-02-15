use crate::{
    board_utils::chessboard::{Board, ChessBoard},
    utils::{Color, Piece},
};
use dfdx::prelude::*;
use crate::utils::Coordinate;

use dfdx::optim::Adam;
use dfdx::tensor::{Cpu};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

///TRAINING FUNCTION ///
#[derive(Clone, Debug)]
pub enum Label {
    WinForWhite(f32),
    AdvantageWhite(f32),
    Draw(f32),
    AdvantageBlack(f32),
    WinForBlack(f32),
}

// 64 squares + 1 for the color to play, 5 outputs for the evaluation (win for white, advantage for white, draw, advantage for black, win for black)
type Model = (Linear<65, 10>, ReLU, Linear<10, 5>);

// This function converts a chess board into a tensor that can be fed into the neural network
fn board_to_tensor(board: &Board) -> Tensor2D<1, 65> {
    let dev = Cpu::default();
    let mut tensor: Tensor2D<1, 65> = dev.zeros();
    for (i, square) in board.squares.iter().enumerate() {
        tensor[[0, i]] = match square.get_piece() {
            Some(piece) => match piece {
                Piece::Pawn(color) => {
                    if *color == Color::White {
                        1.0
                    } else {
                        -1.0
                    }
                }
                Piece::Knight(color) => {
                    if *color == Color::White {
                        2.0
                    } else {
                        -2.0
                    }
                }
                Piece::Bishop(color) => {
                    if *color == Color::White {
                        3.0
                    } else {
                        -3.0
                    }
                }
                Piece::Rook(color) => {
                    if *color == Color::White {
                        4.0
                    } else {
                        -4.0
                    }
                }
                Piece::Queen(color) => {
                    if *color == Color::White {
                        5.0
                    } else {
                        -5.0
                    }
                }
                Piece::King(color) => {
                    if *color == Color::White {
                        6.0
                    } else {
                        -6.0
                    }
                }
            },
            None => 0.0,
        };
    }
    tensor[[0, 64]] = match board.color_to_play() {
        Color::White => 1.0,
        Color::Black => -1.0,
    };
    // Normalize features to roughly [-1, 1] (original values are in [-6,6])
    for i in 0..65 {
        tensor[[0, i]] /= 6.0;
    }
    tensor
}

pub fn extract_data(file_path: &str) -> Result<Vec<(Board, Label)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut records = Vec::new();

    for line in contents.lines().into_iter().skip(1) {
        let record: Vec<String> = line
            .split(',')
            .enumerate()
            .filter_map(|(i, s)| {
                if i == 1 || i==2 || i == 7 {
                    Some(s.to_string())
                } else {
                    None
                }
            })
            .collect();

        records.append(&mut string_to_Label(record));
    }
    Ok(records)
}

fn string_to_Label(s: Vec<String>) -> Vec<(Board, Label)> {
    let mut board = Board::from_fen(&s[0]).unwrap();
    let player_color = board.color_to_play();
    
    let mut data : Vec<(Board, Label)> = Vec::new();
    let moves = s[1].split_whitespace().collect::<Vec<&str>>();

    let boards : Vec<Board> = Vec::new();

    let theme = &s[2];
    let lab : Label =  if theme.contains("equality"){
        Label::Draw(1.0)
    } else { 
        match player_color {
        Color::White => {
            if theme.contains("mate") || theme.contains("Mate") {
                Label::WinForWhite(1.0)
            } else {
                Label::AdvantageWhite(1.0)
            }
        }
        Color::Black => {
            if theme.contains("mate") || theme.contains("Mate") {
                Label::WinForBlack(1.0)
            } else {
                Label::AdvantageBlack(1.0)
            }
        }
    }
    };
    for mov in moves.iter(){
        data.push((board, lab.clone()));
        let from = Coordinate::from(&mov[0..2]);
        let to = Coordinate::from(&mov[2..4]);
        board = board.implement_move_board(from, to);
    }
    data
}

pub fn train_model( data: &Vec<(Board, Label)>, epochs: usize) {
    let dev = Cpu::default();

    let mut model = dev.build_module::<Model, f32>();
    // 1. allocate gradients for the model
    let mut grads = model.alloc_grads();
    // 2. create our optimizer
    let mut opt = Adam::new(&model, AdamConfig{
        lr: 1e-4,
        betas: [0.9, 0.999],
        eps: 1e-8,
        weight_decay: Some(WeightDecay::Decoupled(1e-6)),
    });
    // 3. trace gradients through forward pass
    for _epoch in 0..epochs {
        for (i,(board, label)) in data.into_iter().enumerate() {
            let x = board_to_tensor(board);

            let y = model.forward_mut(x.traced(grads));
            // 4. compute loss & run backpropagation


            let mut target: Tensor<(Const<1>, Const<5>), f32, _> = dev.zeros();
            match label {
                Label::WinForWhite(_) => target[[0, 0]] = 1.0,
                Label::AdvantageWhite(_) => target[[0, 1]] = 1.0,
                Label::Draw(_) => target[[0, 2]] = 1.0,
                Label::AdvantageBlack(_) => target[[0, 3]] = 1.0,
                Label::WinForBlack(_) => target[[0, 4]] = 1.0,
            }

            // cross entropy (stable)
             if i % 1e5 as usize == 0 {
                print!("Epoch: {}, iter {}, y: {:?} ", _epoch,i, &y.as_vec());
            }
            let log_probs = y.log_softmax::<Axis<1>>();

            let loss = -(log_probs * target).sum();

           
            //let loss = (y_sig - target).square().mean();
            // debug: print loss occasionally
            if i % 1e5 as usize == 0 {
                println!(" loss: {:?}", loss.as_vec());
            }
            grads = loss.backward();
            // 5. apply gradients
            let _ = opt.update(&mut model, &grads);
        }
    }
    //save the model to a file
    print!("Training complete. Saving model...");
    let save = model.save_safetensors(r"models\trained_on_puzzles.safetensors");
    println!("Model saved: {:?}", save);
}

/// POST TRAINING EVALUATION FUNCTION ///
pub fn evaluate_model(path: &str, data: &Vec<(Board, Label)>) -> f32{
    let dev = Cpu::default();
    let mut model = dev.build_module::<Model, f32>();
    // Load the trained model from file
    match model.load_safetensors(path) {
        Ok(_) => println!("Model loaded successfully from {}", path),
        Err(e) => {
            println!("Failed to load model from {}: {:?}", path, e);
            return 0.0;
        }
    }
    let mut accuracy = 0;
    for (board, label) in data.iter() {
        let input = board_to_tensor(board);
        let output = model.forward(input);
        let column_output = output.as_vec();
        //println!("Board: {:?}, Label: {:?}, Output: {:?}", board, label, column_output);
        let predicted_label = column_output
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        let true_label = match label {
            Label::WinForWhite(_) => 0,
            Label::AdvantageWhite(_) => 1,      
            Label::Draw(_) => 2,
            Label::AdvantageBlack(_) => 3,
            Label::WinForBlack(_) => 4,
        };
        if predicted_label == true_label {
            accuracy += 1;
        }

    }
    let final_accuracy = (accuracy as f32)/ (data.len() as f32);
    println!("Final accuracy: {:.2}%", final_accuracy * 100.0);
    println!("Number of samples evaluated: {}", data.len());
    final_accuracy
}

pub fn evaluate_board(path: &str, board: &Board) -> Vec<f32>{
    let dev = Cpu::default();
    let mut model = dev.build_module::<Model, f32>();
    // Load the trained model from file
    match model.load_safetensors(path) {
        Err(e) => {
            println!("Failed to load model from {}: {:?}", path, e);
            return [0.0,0.0,0.0,0.0,0.0].to_vec();
        }
        Ok(_) => {},
        // println!("Model loaded successfully from {}", path),
    }
    let mut accuracy = 0;
    let input = board_to_tensor(&board);
    let output = model.forward(input);
    return output.as_vec();
}

