use arrow::array::RecordBatch;
use arrow_json::reader::ReaderBuilder;
use arrow_schema::{DataType, Field, Schema};
use dateparser::DateTimeUtc;
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyValueError;
use pyo3::PyErr;
use pyo3::PyResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Games {
    games: Vec<Game>,
}

impl TryFrom<Games> for RecordBatch {
    type Error = PyErr;

    fn try_from(other: Games) -> Result<Self, Self::Error> {
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
        let schema = Schema::new(vec![
            Field::new("avatar", DataType::Utf8, false),
            Field::new("player_id", DataType::Int32, false),
            Field::new("@id", DataType::Utf8, false),
            Field::new("url", DataType::Utf8, true),
            Field::new("name", DataType::Utf8, false),
            Field::new("username", DataType::Utf8, false),
            Field::new("title", DataType::Utf8, true),
            Field::new("followers", DataType::Int32, false),
            Field::new("country", DataType::Utf8, false),
            Field::new("location", DataType::Utf8, false),
            Field::new("last_online", DataType::Int64, false),
            Field::new("joined", DataType::Int64, false),
            Field::new("status", DataType::Utf8, false),
            Field::new("is_streamer", DataType::Boolean, false),
            Field::new("verified", DataType::Boolean, false),
            Field::new("league", DataType::Utf8, false),
        ]);
        let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().map_err(|error|PyException::new_err(format!("Error with formatting when conv when converting schema input to arrow schema: {}", error)))?;
        decoder.serialize(&other.games).map_err(|error|PyException::new_err(format!("Error with serializing the payloads when conv when converting schema input to arrow schema: {}", error)))?;
        let batch = decoder
            .flush()
            .map_err(|error| {
                PyException::new_err(format!("Error when flushing pyarrow batch: {}", error))
            })?
            .ok_or(PyValueError::new_err("Resulting pyarrow object was empty"))?;
        Ok(batch)
    }
}

impl Games {
    pub fn new(client: &PyClient, archives: PlayersArchives) -> PyResult<Self> {
        let mut games = Games { games: vec![] };

        for player_archive in archives.players {
            for archive_url in player_archive.archives {
                let res = client.get_url(&archive_url)?;
                let player_games = serde_json::from_str::<Games>(&res).map_err(|error| {
                    PyValueError::new_err(format!(
                        "Error in parsing the payload into a game. {}",
                        error
                    ))
                })?;
                games.games.extend(player_games.games);
            }
        }
        Ok(games)
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Game {
    url: Option<String>,
    pgn: Option<String>,
    time_control: String,
    end_time: i64,
    rated: bool,
    accuracies: Option<Accuracies>,
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
