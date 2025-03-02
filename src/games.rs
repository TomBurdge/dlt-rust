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

// DUCKDB existing schema:
// ┌───────────────────┬──────────────────────────┬─────────┬─────────┬─────────┬─────────┐
// │    column_name    │       column_type        │  null   │   key   │ default │  extra  │
// │      varchar      │         varchar          │ varchar │ varchar │ varchar │ varchar │
// ├───────────────────┼──────────────────────────┼─────────┼─────────┼─────────┼─────────┤
// │ end_time          │ TIMESTAMP WITH TIME ZONE │ YES     │ NULL    │ NULL    │ NULL    │
// │ url               │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ pgn               │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ time_control      │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ rated             │ BOOLEAN                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ accuracies__white │ DOUBLE                   │ YES     │ NULL    │ NULL    │ NULL    │
// │ accuracies__black │ DOUBLE                   │ YES     │ NULL    │ NULL    │ NULL    │
// │ tcn               │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ uuid              │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ initial_setup     │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ fen               │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ time_class        │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ rules             │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ white__rating     │ BIGINT                   │ YES     │ NULL    │ NULL    │ NULL    │
// │ white__result     │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ white__aid        │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ white__username   │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ white__uuid       │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ black__rating     │ BIGINT                   │ YES     │ NULL    │ NULL    │ NULL    │
// │ black__result     │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ black__aid        │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ black__username   │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ black__uuid       │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ eco               │ VARCHAR                  │ YES     │ NULL    │ NULL    │ NULL    │
// │ _dlt_load_id      │ VARCHAR                  │ NO      │ NULL    │ NULL    │ NULL    │
// │ _dlt_id           │ VARCHAR                  │ NO      │ NULL    │ NULL    │ NULL    │
// ├───────────────────┴──────────────────────────┴─────────┴─────────┴─────────┴─────────┤
// │ 26 rows                                                                    6 columns │
#[derive(Deserialize, Serialize, Debug)]
struct Game {
    url: String,
    pgn: String,
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
