use dateparser::DateTimeUtc;
use pyo3::exceptions::PyValueError;
use pyo3::PyResult;

pub fn month_string_to_date(string: &str) -> PyResult<DateTimeUtc> {
    let date = string.parse::<DateTimeUtc>().map_err(|err| {
        PyValueError::new_err(format!(
            "{} could not be parsed date to a date object.",
            err
        ))
    })?;
    Ok(date)
}
