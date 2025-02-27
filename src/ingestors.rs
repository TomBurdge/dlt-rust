use arrow::array::RecordBatch;
use arrow::pyarrow::PyArrowType;
use pyo3::prelude::*;
use pyo3::pyfunction;

use super::records;

#[pyfunction]
pub fn get_player_profiles(players: Vec<String>) -> PyResult<PyArrowType<RecordBatch>> {
    let mut results = records::PlayerPayloads::new();
    for player in players {
        let player = records::get_player_profile(player)?;
        results.push_payload(player);
    }
    let results = results.try_into()?;
    Ok(PyArrowType(results))
}
