use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::PyResult;
use reqwest::blocking::Client;

#[pyclass]
pub struct PyClient {
    #[allow(dead_code)]
    client: Client,
}

#[pymethods]
impl PyClient {
    #[new]
    fn new() -> PyResult<Self> {
        let res = Client::builder()
            .build()
            .map_err(|error| PyException::new_err(format!("Could not build client: {}", error)))?;
        Ok(PyClient { client: res })
    }
}
