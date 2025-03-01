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
    start_month: DateTimeUtc,
    end_month: DateTimeUtc,
}

impl PlayersArchives {
    pub fn new(start_month: DateTimeUtc, end_month: DateTimeUtc) -> PyResult<Self> {
        if end_month.0 < start_month.0 {
            return Err(PyValueError::new_err(
                "End month cannot be later than start month.",
            ));
        }
        Ok(PlayersArchives {
            players: vec![],
            start_month,
            end_month,
        })
    }

    fn add_player_archive(&mut self, mut player: PlayerArchives) -> PyResult<()> {
        player.archives = player
            .archives
            .into_iter()
            .map(|s| {
                let dt = s[6..].parse::<DateTimeUtc>().map_err(|err| {
                    PyValueError::new_err(format!(
                        "Response body of function could not be parsed to a date object: {}",
                        err
                    ))
                })?;
                Ok::<(std::string::String, DateTimeUtc), pyo3::PyErr>((s, dt))
            })
            .collect::<Result<Vec<(String, DateTimeUtc)>, _>>()?
            .into_iter()
            .filter_map(|(s, dt)| {
                if self.start_month.0 <= dt.0 && dt.0 <= self.end_month.0 {
                    Some(s)
                } else {
                    None
                }
            })
            .collect();
        self.players.push(player);
        Ok(())
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
        archives.add_player_archive(res)?;
    }
    Ok(archives)
}
