use arrow::array::RecordBatch;
use arrow_json::reader::ReaderBuilder;
use arrow_schema::{DataType, Field, Schema};
use pyo3::exceptions::PyException;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::client::PyClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfile {
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

pub struct PlayerProfiles {
    payloads: Vec<PlayerProfile>,
    schema: Schema,
}

impl Default for PlayerProfiles {
    fn default() -> Self {
        PlayerProfiles::new()
    }
}

impl PlayerProfiles {
    pub fn new() -> Self {
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

        PlayerProfiles {
            payloads: vec![],
            schema,
        }
    }

    pub fn push_payload(&mut self, payload: PlayerProfile) {
        self.payloads.push(payload)
    }
}

impl TryFrom<PlayerProfiles> for RecordBatch {
    type Error = PyErr;

    fn try_from(other: PlayerProfiles) -> Result<Self, Self::Error> {
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

pub fn get_player_profile(client: &PyClient, username: String) -> PyResult<PlayerProfile> {
    let path = format!("player/{}", username);
    let url = format!("{}{}", super::OFFICIAL_CHESS_API_URL, path);
    let res = client.get_url(&url)?;
    let payload = serde_json::from_str::<PlayerProfile>(&res).map_err(|error| {
        PyValueError::new_err(format!("Error in parsing the payload {}", error))
    })?;
    Ok(payload)
}
