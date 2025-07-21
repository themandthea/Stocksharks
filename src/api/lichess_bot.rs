use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::error::Error;

pub fn send_move(game_id: &str, move_uci: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://lichess.org/api/bot/game/{}/move/{}", game_id, move_uci);

    let client = Client::new();
    let res = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()?;

    if res.status().is_success() {
        println!("Coup envoyé avec succès !");
        Ok(())
    } else {
        println!("Erreur lors de l'envoi du coup : {:?}", res.text()?);
        Err("Erreur API".into())
    }
}

pub fn get_game_ids(token: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = "https://lichess.org/api/account/playing";
    let client = Client::new();
    let res = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()?;

    if res.status().is_success() {
        let body = res.text()?;
        let json: serde_json::Value = serde_json::from_str(&body)?;
        let mut ids = Vec::new();
        if let Some(now_playing) = json.get("nowPlaying").and_then(|v| v.as_array()) {
            for game in now_playing {
                if let Some(game_id) = game.get("gameId").and_then(|v| v.as_str()) {
                    ids.push(game_id.to_string());
                }
            }
        }
        Ok(ids)
    } else {
        Err(format!("Erreur lors de la récupération des parties : {:?}", res.text()?).into())
    }
}


pub fn get_game_fen(game_id: &str, token: &str) -> Result<String, Box<dyn Error>> {
    let url = "https://lichess.org/api/account/playing";
    let client = Client::new();
    let res = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()?;

    if res.status().is_success() {
        let body = res.text()?;
        let json: serde_json::Value = serde_json::from_str(&body)?;
        
        if let Some(now_playing) = json.get("nowPlaying").and_then(|v| v.as_array()) {
            for game in now_playing {
                if let Some(id) = game.get("gameId").and_then(|v| v.as_str()) {
                    if id == game_id {
                        if let Some(fen) = game.get("fen").and_then(|f| f.as_str()) {
                            //println!("FEN: {}", fen);
                            return Ok(fen.to_string());
                        }
                    }
                }
            }
        }
        
        Err("Partie ou notation FEN non trouvée dans la réponse".into())
    } else {
        Err(format!("Erreur lors de la récupération de la position FEN : {:?}", res.text()?).into())
    }
}