use dateparser::DateTimeUtc;
use pyo3::exceptions::PyValueError;
use pyo3::PyResult;
use serde::{Deserialize, Serialize};

use super::archives::PlayersArchives;
use super::client::PyClient;

pub fn month_string_to_date(string: &str) -> PyResult<DateTimeUtc> {
    let date = string.parse::<DateTimeUtc>().map_err(|err| {
        PyValueError::new_err(format!(
            "{} could not be parsed date to a date object.",
            err
        ))
    })?;
    Ok(date)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Games {
    games: Vec<Game>,
}

impl Games {
    pub fn new(_client: &PyClient, _archives: PlayersArchives) -> PyResult<Self> {
        todo!("Add Games creator/getter process")
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Game {
    url: String,
    time_control: String,
    end_time: i64,
    rated: bool,
    accuracies: Accuracies,
    tcn: String,
    uuid: String,
    initial_setup: String,
    time_class: String,
    rules: String,
    white: PlayerInformation,
    black: PlayerInformation,
    eco: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Accuracies {
    white: f32,
    black: f32,
}

#[derive(Deserialize, Serialize, Debug)]
struct PlayerInformation {
    rating: i32,
    result: String,
    #[serde(rename = "@id")]
    id: String,
    username: String,
    uuid: String,
}
