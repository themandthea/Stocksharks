use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::env;

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

