use crate::client::PyClient;
use dateparser::DateTimeUtc;
use pyo3::exceptions::PyValueError;
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerArchives {
    pub archives: Vec<String>,
}

#[derive(Debug)]
pub struct PlayersArchives {
    pub players: Vec<PlayerArchives>,
    #[allow(dead_code)]
    start_month: DateTimeUtc,
    #[allow(dead_code)]
    end_month: DateTimeUtc,
}

impl PlayersArchives {
    pub fn new(start_month: DateTimeUtc, end_month: DateTimeUtc) -> PyResult<Self> {
        Ok(PlayersArchives {
            players: vec![],
            start_month,
            end_month,
        })
    }

    pub fn filter(&mut self) -> Self {
        todo!("Filter the dates with a date strings.")
    }
}

pub fn get_player_archives(
    client: &PyClient,
    players: Vec<String>,
    start_month: DateTimeUtc,
    end_month: DateTimeUtc,
) -> PyResult<PlayersArchives> {
    let mut archives = PlayersArchives::new(start_month, end_month)?;
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
