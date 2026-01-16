pub mod utils;
pub mod piece;
pub mod board_utils;
pub mod api;
pub mod ai;
use crate::board_utils::chessboard::{Board, ChessBoard};
use crate::utils::{Coordinate, Piece, Square};
use crate::ai::alpha_beta;

use std::time::Instant;
use std::thread;
use std::env;
use std::time::Duration;

fn main() {
    use crate::api::lichess_bot::{challenge_player, get_game_ids, get_game_fen, send_move, get_bot_color};

    let token = env::var("LICHESS_TOKEN").expect("Définis la variable d'environnement LICHESS_TOKEN");

    // 1. Envoyer le défi
    if let Err(e) = challenge_player("yodavsshrek", &token) {
        eprintln!("Erreur lors de l'envoi du défi : {}", e);
        return;
    }
    println!("Défi envoyé à yodavsshrek, en attente d'acceptation...");

    // 2. Attendre que la partie commence et récupérer l'ID
    let game_id = loop {
        thread::sleep(Duration::from_secs(2));
        let ids = get_game_ids(&token).unwrap_or_default();
        if let Some(id) = ids.first() {
            println!("Partie trouvée ! game_id: {}", id);
            break id.clone();
        }
        println!("En attente que la partie commence...");
    };

    // 3. Récupérer la couleur du bot
    let my_color = get_bot_color(&game_id, &token).unwrap().unwrap();
    println!("Je joue la couleur : {:?}", my_color);

    // 4. Boucle de jeu
    loop {
        let fen = get_game_fen(&game_id, &token).unwrap();
        let board = Board::from_fen(&fen).unwrap();

        // Si ce n'est pas à nous de jouer, on attend
        if board.color_to_play() != my_color {
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        print!("{}\n", board);

        let start = Instant::now();

        let ((init_pos, dest_pos), value) = alpha_beta::alpha_beta_root(&board, 4);

        let duration = start.elapsed();
        println!("Temps d'exécution pour alpha_beta : {:?}", duration);

        let mov = board.get_uci_move(&init_pos, &dest_pos);
        println!("Coup choisi: {:?} avec une valeur de {:?}", mov, value);

        if let Some(mov) = mov {
            if let Err(e) = send_move(&game_id, &mov, &token) {
                eprintln!("Erreur : {}", e);
            }
        } else {
            println!("Aucun coup valide trouvé, partie terminée ?");
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

/* 
    let puzzle = "8/8/7K/8/8/5QR1/2k5/8 w - - 0 1".to_string();
    let mut board = Board::from_fen(&puzzle).unwrap();
    loop{

    println!("{}",board);
    thread::sleep(Duration::from_secs(2)); // Attendre 0.5 seconde avant de continuer
    let ((init_pos,dest_pos), value) = alpha_beta::alpha_beta(&board, 4, Evaluate::MateForBlack(0), Evaluate::MateForWhite(0));
    println!("Coup choisi: {:?} avec une valeur de {:?}", dest_pos, value);
    if dest_pos.get_line().is_none() {
        println!("Pas de coup possible, checkmate ou pat.");
        break; // On attend le coup de l'adversaire
    }
    println!("color to play: {:?}", board.color_to_play());
    board = board.implement_move_board(init_pos.coordinate(), dest_pos.coordinate());
    println!("color to play: {:?}", board.color_to_play());
*/