use pyo3::prelude::PyModule;
use pyo3::pymodule;
use pyo3::types::PyModuleMethods;
use pyo3::wrap_pyfunction;
use pyo3::Bound;
use pyo3::PyResult;

pub mod client;
pub mod games;
pub mod ingestors;
pub mod records;

const OFFICIAL_CHESS_API_URL: &str = "https://api.chess.com/pub/";

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ingestors::get_player_profiles, m)?)?;
    m.add_class::<client::PyClient>()?;
    Ok(())
}
