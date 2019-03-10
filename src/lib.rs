//! An Embedded Time Series Database
//!
//! This library provides a low-intensity time series database meant to be embedded inside of an
//! application. The database is implemented as an append-only JSON file.
#[macro_use]
extern crate serde_derive;

mod criteria;
mod series;
mod types;

pub use types::{Error, Record, Recordable, UniqueId};
pub use series::Series;
pub use criteria::*;
