use pyo3::exceptions::PyValueError;
use pyo3::PyResult;

// def validate_month_string(string: str) -> None:
//     """Validates that the string is in YYYY/MM format"""
//     if string and string[4] != "/":
//         raise ValueError(string)

#[allow(dead_code)]
fn validate_month_string(string: String) -> PyResult<()> {
    let char_4 = string.chars().nth(4).ok_or_else(|| {
        PyValueError::new_err(format!(
            "Input date string {} was not long enough to be a valid date.",
            string
        ))
    })?;
    if char_4 == '/' {
        return Err(PyValueError::new_err(
            "Fourth char of date string input was '/'. This is not a validate format.",
        ));
    }
    Ok(())
}
