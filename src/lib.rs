/*
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate uuid;
*/

//use chrono::prelude::*;
//use std::fs::File;
//use std::io::{ [> BufRead, <] BufWriter };
//use uuid::{ Uuid };

mod criteria;
mod series;
mod types;

pub use types::{ Error, Record, Recordable };
pub use series::{ Series };
pub use criteria::{ Criteria, exact_time };

/* A time series needs to read the entire data stream. It also needs to be able to read a single
 * record. This is really just mapping over a line-oriented stream.
 *
 * My write pattern requires that the output file be kept open indefinitely.
 *
 * My read pattern requires that the input file be open just long enough to read everything, unless
 * I decide to start chunking data. And I think I'm going to avoid that for now.
 */


