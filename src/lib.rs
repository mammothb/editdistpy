mod common;
mod damerau_osa;
mod levenshtein;

#[cfg(feature = "python")]
mod editdistpy_rs;

#[cfg(feature = "python")]
pub use editdistpy_rs::_editdistpy;
