use crate::client::PyClient;
use pyo3::exceptions::PyValueError;
use pyo3::PyResult;
use serde::{Deserialize, Serialize};
// use arrow_schema::{Field, Schema, DataType};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerArchives {
    pub archives: Vec<String>,
}

#[derive(Debug)]
pub struct PlayersArchives {
    pub players: Vec<PlayerArchives>,
    // schema: Schema
}

impl Default for PlayersArchives {
    fn default() -> Self {
        PlayersArchives::new()
    }
}

impl PlayersArchives {
    pub fn new() -> Self {
        PlayersArchives {
            players: vec![], // schema: Schema::new(vec![
                             //     Field::new("archives", DataType::List(), )
                             // ])
        }
    }

    pub fn filter(&mut self) -> Self {
        todo!("Filter the dates with a date strings.")
    }
}

pub fn get_player_archives(client: &PyClient, players: Vec<String>) -> PyResult<PlayersArchives> {
    let mut archives = PlayersArchives::new();
    for player in players {
        let path = format!("player/{}/games/archives", player);
        let url = format!("{}{}", super::OFFICIAL_CHESS_API_URL, path);
        let res = client.get_url(&url)?;
        let res = serde_json::from_str::<PlayerArchives>(&res).map_err(|error| {
            PyValueError::new_err(format!("Error in parsing the payload {}", error))
        })?;
        archives.players.push(res);
    }
    Ok(archives)
}
