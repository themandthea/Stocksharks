pub mod utils;
pub mod piece;
pub mod board;
pub mod api;
use crate::board::Board;
use crate::utils::{Coordinate, Piece, Square, Color};
use crate::board::ChessBoard;
use std::env;
use crate::api::lichess_bot::{send_move, get_game_ids, get_game_fen};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::thread;
use std::time::Duration;

use rand::prelude::IndexedRandom;


fn main() {

        // Récupère les infos depuis les variables d'environnement ou remplace par tes valeurs
    let token = env::var("LICHESS_TOKEN").expect("Définis la variable d'environnement LICHESS_TOKEN");
    let binding = get_game_ids(&token).unwrap();
    let game_id = binding.first(); // Remplace par l'ID de la partie
    println!("game_id: {:?}", game_id);

    let binding = "09GD1BqM".to_string();
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

        let legal_moves = board.legal_moves_uci();

        let mut rng = thread_rng();

        let mov = legal_moves.choose(&mut rng).unwrap();
        println!("Coup choisi au hasard : {:?}", mov);

        if let Err(e) = send_move(id, mov, &token) {
        eprintln!("Erreur : {}", e);
        }
        thread::sleep(Duration::from_millis(500)); // Attendre 0.5 seconde avant de continuer

    }



    let mut board = Board::new();
    /* 
    let _ = board.set_piece(Coordinate::E(0), Some(Piece::King(Color::White)));
    let _ =board.set_piece(Coordinate::E(3), Some(Piece::Knight(Color::White)));
    let _ =board.set_piece(Coordinate::D(0), Some(Piece::Rook(Color::White)));
    let _ =board.set_piece(Coordinate::E(7), Some(Piece::King(Color::Black)));
    let _ = board.set_piece(Coordinate::H(7), Some(Piece::Rook(Color::Black)));
    let _ = board.set_piece(Coordinate::D(6), Some(Piece::Pawn(Color::Black)));
    let _ = board.set_piece(Coordinate::C(5), Some(Piece::Bishop(Color::White)));
    let _ = board.set_piece(Coordinate::C(2), Some(Piece::Bishop(Color::Black)));
    println!("{}", board);  // Utilise l'implémentation du trait Display
*/
    let legalmoves = board.legal_moves().unwrap();


}
