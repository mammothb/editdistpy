mod common;
mod damerau_osa;
mod levenshtein;
mod pmv;

#[cfg(feature = "python")]
mod editdistpy_rs;

#[cfg(feature = "python")]
pub use editdistpy_rs::_editdistpy;
