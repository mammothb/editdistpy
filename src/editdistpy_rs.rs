use crate::damerau_osa;
use crate::levenshtein;
use pyo3::prelude::*;

/// Levenshtein edit distance with optional max_distance cutoff.
///
/// Returns -1 if distance exceeds max_distance.
#[pyfunction]
fn levenshtein_distance(string_1: Option<&str>, string_2: Option<&str>, max_distance: i64) -> i32 {
    levenshtein::distance(string_1, string_2, max_distance)
}

/// Damerau-Levenshtein OSA edit distance with optional max_distance cutoff.
///
/// Returns -1 if distance exceeds max_distance.
#[pyfunction]
fn damerau_osa_distance(string_1: Option<&str>, string_2: Option<&str>, max_distance: i64) -> i32 {
    damerau_osa::distance(string_1, string_2, max_distance)
}

#[pymodule]
pub fn _editdistpy(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein_distance, m)?)?;
    m.add_function(wrap_pyfunction!(damerau_osa_distance, m)?)?;
    Ok(())
}
