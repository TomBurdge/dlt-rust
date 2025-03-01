use arrow::array::RecordBatch;
use arrow::pyarrow::PyArrowType;
use pyo3::prelude::*;
use pyo3::pyfunction;

use super::archives;
use super::client::PyClient;
use super::games;
use super::records;

#[pyfunction]
pub fn get_player_profiles(
    client: &PyClient,
    players: Vec<String>,
) -> PyResult<PyArrowType<RecordBatch>> {
    let mut results = records::PlayerPayloads::new();
    for player in players {
        let player = records::get_player_profile(client, player)?;
        results.push_payload(player);
    }
    let results = results.try_into()?;
    Ok(PyArrowType(results))
}

#[pyfunction]
pub fn get_player_games(
    client: &PyClient,
    players: Vec<String>,
    start_month: String,
    end_month: String,
) -> PyResult<()> {
    let start_month = games::month_string_to_date(&start_month)?;
    let end_month = games::month_string_to_date(&end_month)?;
    let archives = archives::get_player_archives(client, players, start_month, end_month)?;
    println!(
        "{}",
        archives
            .players
            .into_iter()
            .next()
            .unwrap()
            .archives
            .into_iter()
            .next()
            .unwrap()
    );
    Ok(())
}
