extern crate serde;

use serde::de;
use serde::de::{ Visitor };
use std::fmt;

pub struct F64Visitor;

impl <'de> Visitor<'de> for F64Visitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 64-bit floating point value")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(v)
    }
}


