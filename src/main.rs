pub mod utils;
pub mod piece;
pub mod board_utils;
pub mod api;
pub mod ai;
use crate::board_utils::chessboard::{Board, ChessBoard};
use crate::utils::{Coordinate, Piece, Square, Color};
use crate::api::lichess_bot::{send_move, get_game_fen, get_game_ids};
use crate::ai::alpha_beta;



use std::thread;
use std::env;
 use std::time::Duration;
   
fn main() {

        // Récupère les infos depuis les variables d'environnement ou remplace par tes valeurs
    let token = env::var("LICHESS_TOKEN").expect("Définis la variable d'environnement LICHESS_TOKEN");
    let binding = get_game_ids(&token).unwrap();
    let game_id = binding.first(); // Remplace par l'ID de la partie
    println!("game_id: {:?}", game_id);

    let binding = "OZZQHwNj".to_string();
    let id = game_id.unwrap_or(&binding);

    let mut fen = get_game_fen(id, &token).unwrap();
    println!("FEN: {:?}", fen);
    
    loop {
        fen = get_game_fen(id, &token).unwrap();
        let board = Board::from_fen(&fen).unwrap();
        if board.color_to_play() == Color::White {
            println!("C'est le tour des noirs, on attend le coup de l'adversaire...");
            continue; // On attend le coup de l'adversaire
        }
        print!("{}\n", board);

        let (mov,value)  = alpha_beta::alpha_beta(&board, 3,i32::MIN,i32::MAX, true);
        println!("Coup choisi: {:?} avec une valeur de {}", mov, value);
        if let Err(e) = send_move(id, &mov, &token) {
        eprintln!("Erreur : {}", e);
        }
        thread::sleep(Duration::from_millis(500)); // Attendre 0.5 seconde avant de continuer

    }
}
