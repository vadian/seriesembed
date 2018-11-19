extern crate chrono;
extern crate dimensioned;
extern crate emseries;
extern crate serde;
#[macro_use] extern crate serde_derive;

use chrono::prelude::*;
use dimensioned::si::{ KG, Kilogram };
use emseries::{ Error, Record, Recordable, Series, UniqueId };
//use serde::de;
use serde::de::{ Deserialize, Deserializer };
use serde::ser::{ Serialize, Serializer };

mod helpers;
use helpers::F64Visitor;


#[derive(Clone, Debug, PartialEq)]
pub struct Weight(Kilogram<f64>);

impl <'de> Deserialize<'de> for Weight {
    fn deserialize<D>(deserializer: D) -> Result<Weight, D::Error>
        where D: Deserializer<'de>
    {
        let val = deserializer.deserialize_f64(F64Visitor)?;
        Ok(Weight(val * KG))
    }
}

impl Serialize for Weight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_f64(self.0.value_unsafe)
    }
}



#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WeightRecord {
    pub date: DateTime<Utc>,
    pub weight: Weight,
}

impl Recordable for WeightRecord {
    fn timestamp(&self) -> DateTime<Utc> {
        self.date
    }

    fn tags(&self) -> Vec<String> {
        Vec::new()
    }

    fn values(&self) -> Vec<String> {
        Vec::new()
    }
}



const WEIGHT_ENTRY: &str = "{\"data\":{\"weight\":77.79109,\"date\":\"2003-11-10T06:00:00.000000000000Z\"},\"id\":\"3330c5b0-783f-4919-b2c4-8169c38f65ff\"}";

#[test]
pub fn legacy_deserialization() {
    let rec: Result<Record<WeightRecord>, Error> = Series::parse_line(WEIGHT_ENTRY);
    match rec {
        Err(err) => assert!(false, err),
        Ok(rec_) => println!("[decoded record] {:?}", rec_),
    }
}

#[test]
pub fn legacy_file_load() {
    let ts: Series<WeightRecord> = Series::open("fixtures/weight.json").expect("legacy series should open correctly");

    for record in ts.all_records().expect("all_records should never fail") {
        println!("[Record] {:?}", record);
    }

    let uid = UniqueId::from_str("3330c5b0-783f-4919-b2c4-8169c38f65ff").expect("something is wrong with this ID");
    let rec = ts.get(&uid);
    match rec {
        Err(err) => assert!(false, err),
        Ok(None) => assert!(false, "no record found"),
        Ok(Some(rec)) => assert_eq!(rec.data.weight, Weight(77.79109 * KG)),
    }
}

