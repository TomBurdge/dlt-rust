use arrow::array::RecordBatch;
use arrow::pyarrow::PyArrowType;
use pyo3::prelude::*;
use pyo3::pyfunction;

use super::client::PyClient;
use super::games;
use super::records;

#[pyfunction]
pub fn get_player_profiles(
    client: &PyClient,
    players: Vec<String>,
) -> PyResult<PyArrowType<RecordBatch>> {
    let mut results = records::PlayerPayloads::new();
    // TODO: move the client into a pyclass and arguent
    for player in players {
        let player = records::get_player_profile(client, player)?;
        results.push_payload(player);
    }
    let results = results.try_into()?;
    Ok(PyArrowType(results))
}

#[pyfunction]
pub fn get_games(_players: Vec<String>, start_month: String, end_month: String) -> PyResult<()> {
    games::validate_month_string(&start_month)?;
    games::validate_month_string(&end_month)?;
    Ok(())
}
