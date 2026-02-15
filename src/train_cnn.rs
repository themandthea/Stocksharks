
pub mod ai;
pub mod board_utils;
pub mod piece;
pub mod utils;
use crate::ai::cnn::train_model;
use crate::ai::cnn::extract_data;
use crate::board_utils::chessboard::Board;
use crate::ai::cnn::evaluate_model;

pub fn main(){
    
    let data = extract_data(r"C:\Users\mahel\source\repos\Stocksharks\data\lichess_db_puzzle.csv").expect("Failed to extract data");
    println!("Data extracted successfully. Total samples: {}", data.len());
    let (_,mut test_data) = data.split_at((data.len() as f32 * 0.8) as usize);
    //let train = train_data.to_vec();
    let test = test_data.to_vec();
    
    /* 
    println!("starting training...");
    train_model(train,1);
    */

    let evaluation = evaluate_model(r"models\trained_on_puzzles.safetensors",&test);
    
}