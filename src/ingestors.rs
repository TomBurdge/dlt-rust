use arrow::array::RecordBatch;
use arrow::pyarrow::PyArrowType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyfunction;
use reqwest::blocking::Client;

use super::records;

#[pyfunction]
pub fn get_player_profiles(players: Vec<String>) -> PyResult<PyArrowType<RecordBatch>> {
    let mut results = records::PlayerPayloads::new();
    let client = Client::builder()
        .build()
        .map_err(|error| PyValueError::new_err(format!("Could not build client: {}", error)))?;
    for player in players {
        let player = records::get_player_profile(&client, player)?;
        results.push_payload(player);
    }
    let results = results.try_into()?;
    Ok(PyArrowType(results))
}
