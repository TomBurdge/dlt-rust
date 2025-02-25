use arrow::array::RecordBatch;
use arrow::pyarrow::PyArrowType;
use arrow_json::reader::ReaderBuilder;
use arrow_schema::{DataType, Field, Schema};
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const OFFICIAL_CHESS_API_URL: &str = "https://api.chess.com/pub/";

#[derive(Debug, Deserialize, Serialize)]
struct PlayerPayload {
    avatar: String,
    player_id: i32,
    #[serde(rename = "@id")]
    id: String,
    url: String,
    name: String,
    username: String,
    title: Option<String>,
    followers: i32,
    country: String,
    location: String,
    // TODO: parse as date
    // or could do it in pyarrow
    last_online: i64,
    joined: i64,
    status: String,
    is_streamer: bool,
    verified: bool,
    league: String,
    streaming_platforms: Vec<String>,
}

struct PlayerPayloads {
    payloads: Vec<PlayerPayload>,
    schema: Schema,
}

impl PlayerPayloads {
    fn new() -> Self {
        let schema = Schema::new(vec![
            Field::new("avatar", DataType::Utf8, false),
            Field::new("player_id", DataType::Int32, false),
            Field::new("@id", DataType::Utf8, false),
            Field::new("url", DataType::Utf8, false),
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

        PlayerPayloads {
            payloads: vec![],
            schema,
        }
    }

    fn push_payload(&mut self, payload: PlayerPayload) {
        self.payloads.push(payload)
    }
}

impl TryFrom<PlayerPayloads> for RecordBatch {
    type Error = PyErr;

    fn try_from(other: PlayerPayloads) -> Result<Self, Self::Error> {
        let mut decoder = ReaderBuilder::new(Arc::new(other.schema)).build_decoder().map_err(|error|PyException::new_err(format!("Error with formatting when conv when converting schema input to arrow schema: {}", error)))?;
        decoder.serialize(&other.payloads).map_err(|error|PyException::new_err(format!("Error with serializing the payloads when conv when converting schema input to arrow schema: {}", error)))?;
        let batch = decoder
            .flush()
            .map_err(|error| {
                PyException::new_err(format!("Error when flushing pyarrow batch: {}", error))
            })?
            .ok_or(PyValueError::new_err("Resulting pyarrow object was empty"))?;
        Ok(batch)
    }
}

fn get_player_profile(username: String) -> PyResult<PlayerPayload> {
    let path = format!("player/{}", username);
    let url = format!("{}{}", OFFICIAL_CHESS_API_URL, path);
    let client = Client::builder()
        // .cookie_store(true)
        .build()
        .map_err(|error| PyValueError::new_err(format!("Could not build a client: {}", error)))?;
    let res = client
        .get(url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:94.0) Gecko/20100101 Firefox/94.0",
        )
        .header(
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .send()
        .map_err(|error| PyValueError::new_err(format!("Error performing request: {}", error)))?
        .text()
        .expect("body failed");
    let payload = serde_json::from_str::<PlayerPayload>(&res).map_err(|error| {
        PyValueError::new_err(format!("Error in parsing the payload {}", error))
    })?;
    Ok(payload)
}

#[pyfunction]
fn get_player_profiles(players: Vec<String>) -> PyResult<PyArrowType<RecordBatch>> {
    let mut results = PlayerPayloads::new();
    for player in players {
        let player = get_player_profile(player)?;
        results.push_payload(player);
    }
    let results = results.try_into()?;
    Ok(PyArrowType(results))
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_player_profiles, m)?)?;
    Ok(())
}
