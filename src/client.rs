use pyo3::exceptions::PyConnectionError;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::PyResult;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, USER_AGENT};

#[pyclass]
pub struct PyClient {
    pub client: Client,
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

    pub fn get_url(&self, url: &str) -> PyResult<String> {
        let res = self
            .client
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
            .map_err(|error| {
                PyConnectionError::new_err(format!("Error performing request: {}", error))
            })?
            .text()
            .expect("body failed");
        Ok(res)
    }
}
