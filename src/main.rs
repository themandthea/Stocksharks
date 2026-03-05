pub mod ai;
pub mod api;
use chess::{Board, ChessMove, MoveGen, Color};
use serde::de;
use std::str::FromStr;
use std::env;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use crate::ai::heuristic::SimpleHeuristic;
use crate::ai::tree::Node;

fn main() {
    use crate::api::lichess_bot::{
        challenge_player, get_bot_color, get_game_fen, get_game_ids, send_move,
    };

    let token =
        env::var("LICHESS_TOKEN").expect("Définis la variable d'environnement LICHESS_TOKEN");

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
        let board = Board::from_str(&fen).unwrap();

        // Si ce n'est pas à nous de jouer, on attend
        if board.side_to_move() != my_color {
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        println!("{}", board);

        let start = Instant::now();
        let depth = 7; // Profondeur de recherche pour alpha-beta
        let root = Node::from(board);
        let (value,mov) = root.evaluate_path(depth,&SimpleHeuristic{});
        print!("profondeur : {} ", depth);

        let duration = start.elapsed();
        println!("Temps d'exécution pour alpha_beta : {:?}", duration);
        match mov{
            Some(mv) => {
                let mov_str = mv.to_string();
                println!("Coup choisi: {:?} avec une valeur de {:?}", mv, value);
                println!("Envoi du coup {} ", &mov_str);

                if let Err(e) = send_move(&game_id, &mov_str, &token) {
                    eprintln!("Erreur : {}", e);
                }
            },
            None => {println!("Aucun coup légal trouvé, partie terminée."); 
                    continue;
                    }
        }
        
        thread::sleep(Duration::from_secs(1));
    }
}


