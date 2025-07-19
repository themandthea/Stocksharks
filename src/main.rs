pub mod utils;
pub mod piece;
pub mod board;
pub mod api;
use crate::board::Board;
use crate::utils::{Coordinate, Piece, Square, Color};
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::rook::Rook;
use crate::piece::pawn::Pawn;
use crate::api::lichess_bot::send_move;
use crate::board::ChessBoard;

use std::env;

fn main() {

    /* 
        // Récupère les infos depuis les variables d'environnement ou remplace par tes valeurs
    let token = env::var("LICHESS_TOKEN").expect("Définis la variable d'environnement LICHESS_TOKEN");
    let game_id = "votre_game_id"; // Remplace par l'ID de la partie
    let move_uci = "e2e4"; // Remplace par ton coup au format UCI

    if let Err(e) = send_move(game_id, move_uci, &token) {
        eprintln!("Erreur : {}", e);
    }

*/


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

    // Afficher les coups légaux
    legalmoves.iter().for_each(|square| {
        println!("Legal move: \n{}", square);
    });
    

}
