extern crate chrono;
extern crate dimensioned;
extern crate emseries;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use chrono::prelude::*;
use dimensioned::si::{ M, Meter, S, Second };
use std::fmt;
use std::fs;
use std::ops;
use serde::de;
use serde::de::{ Deserialize, Deserializer, Visitor };
use serde::ser::{ Serialize, Serializer, SerializeStruct };
//use uuid::Uuid;

use emseries::*;

#[derive(Clone, Debug, PartialEq)]
struct Distance(Meter<f64>);

struct F64Visitor;

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

impl <'de> Deserialize<'de> for Distance {
    fn deserialize<D>(deserializer: D) -> Result<Distance, D::Error>
        where D: Deserializer<'de>
    {
        let val = deserializer.deserialize_f64(F64Visitor)?;
        Ok(Distance(val * M))
    }
}

impl Serialize for Distance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_f64(self.0.value_unsafe)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Duration(Second<f64>);

impl <'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Duration, D::Error>
        where D: Deserializer<'de>
    {
        let val = deserializer.deserialize_f64(F64Visitor)?;
        Ok(Duration(val * S))
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_f64(self.0.value_unsafe)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct BikeTrip {
    datetime: DateTime<Utc>,
    distance: Distance,
    duration: Duration,
    comments: String,
}

/*
impl Serialize for BikeTrip {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut s = serializer.serialize_struct("BikeTrip", 4)?;
        s.serialize_field("timestamp", &self.datetime)?;
        s.serialize_field("distance", &self.duration.value_unsafe)?;
        s.serialize_field("duration", &self.duration.value_unsafe)?;
        s.serialize_field("comments", &self.comments)?;
        s.end()
    }
}

struct F64Visitor;

impl <'de> Visitor<'de> for DistanceVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 64-bit floating point value")
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(String::from(v))
    }
}


impl <'de> Deserialize<'de> for BikeTrip {
    fn deserialize<D>(deserializer: D) -> Result<BikeTrip, D::Error>
        where D: Deserializer<'de>
    {
        
    }
}
*/

impl Recordable for BikeTrip {
    fn timestamp(&self) -> DateTime<Utc> {
        self.datetime
    }
    fn tags(&self) -> Vec<String> { Vec::new() }
    fn values(&self) -> Vec<String> { Vec::new() }
}

struct SeriesFileCleanup(String);

impl SeriesFileCleanup {
    fn new (path: &str) -> SeriesFileCleanup {
        SeriesFileCleanup(String::from(path))
    }
}

impl ops::Drop for SeriesFileCleanup {
    fn drop (&mut self) {
        fs::remove_file(&self.0).expect("failed to remove time series file");
    }
}

fn mk_trips() -> [BikeTrip; 5] {
    [
        BikeTrip{
            datetime: Utc.ymd(2011, 10, 29).and_hms(0, 0, 0),
            distance: Distance(58741.055 * M),
            duration: Duration(11040.0 * S),
            comments: String::from("long time ago"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 10, 31).and_hms(0, 0, 0),
            distance: Distance(17702.0 * M),
            duration: Duration(2880.0 * S),
            comments: String::from("day 2"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 11, 02).and_hms(0, 0, 0),
            distance: Distance(41842.945 * M),
            duration: Duration(7020.0 * S),
            comments: String::from("Do Some Distance!"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 11, 04).and_hms(0, 0, 0),
            distance: Distance(34600.895 * M),
            duration: Duration(5580.0 * S),
            comments: String::from("I did a lot of distance back then"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 11, 05).and_hms(0, 0, 0),
            distance: Distance(6437.376 * M),
            duration: Duration(960.0 * S),
            comments: String::from("day 5"),
        }
    ]
}


/*
#[test]
pub fn check_serialization() {
    let trips = mk_trips();
    println!("[check_serialization] {:?}", serde_json::to_string(&trips));
    unimplemented!();
}
*/


#[test]
pub fn can_add_and_retrieve_entries() {
    let series_remover = SeriesFileCleanup::new("var/can_add_and_retrieve_entries.json");
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_add_and_retrieve_entries").expect("expect the time series to open correctly");
    let uuid = ts.put(trips[0].clone()).expect("expect a successful put");
    let record_res = ts.get(uuid.clone());

    ts.put(trips[1].clone()).expect("expect a successful put");
    ts.put(trips[2].clone()).expect("expect a successful put");
    ts.put(trips[3].clone()).expect("expect a successful put");
    ts.put(trips[4].clone()).expect("expect a successful put");

    match record_res {
        Err(err) => assert!(false, err),
        Ok(None) => assert!(false, "There should have been a value here"),
        Ok(Some(tr)) => {
            assert_eq!(tr.id, uuid);
            assert_eq!(tr.timestamp(), Utc.ymd(2011, 10, 29).and_hms(0, 0, 0));
            assert_eq!(tr.data.duration, Duration(11040.0 * S));
            assert_eq!(tr.data.comments, String::from("long time ago"));
            assert_eq!(tr.data, trips[0]);
        }
    }
}


#[test]
pub fn can_search_for_an_entry_with_exact_time() {
    let series_remover = SeriesFileCleanup::new("var/can_search_for_an_entry_with_exact_time.json");
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_search_for_an_entry_with_exact_time").expect("expect the time series to open correctly");
    ts.put(trips[0].clone()).expect("expect a successful put");
    ts.put(trips[1].clone()).expect("expect a successful put");
    ts.put(trips[2].clone()).expect("expect a successful put");
    ts.put(trips[3].clone()).expect("expect a successful put");
    ts.put(trips[4].clone()).expect("expect a successful put");

    match ts.search(exact_time(Utc.ymd(2011, 10, 31).and_hms(0, 0, 0))) {
        Err(err) => assert!(false, err),
        Ok(v) => {
            assert_eq!(v.len(), 1);
            assert_eq!(v[0].data, trips[1]);
        }
    }
}


#[test]
pub fn can_get_entries_in_time_range() {
    let series_remover = SeriesFileCleanup::new("var/can_get_entries_in_time_range.json");
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_get_entries_in_time_range").expect("expect the time series to open correctly");
    ts.put(trips[0].clone()).expect("expect a successful put");
    ts.put(trips[1].clone()).expect("expect a successful put");
    ts.put(trips[2].clone()).expect("expect a successful put");
    ts.put(trips[3].clone()).expect("expect a successful put");
    ts.put(trips[4].clone()).expect("expect a successful put");

    match ts.search(time_range(Utc.ymd(2011, 10, 31).and_hms(0, 0, 0), true,
                               Utc.ymd(2011, 11, 04).and_hms(0, 0, 0), true)) {
        Err(err) => assert!(false, err),
        Ok(v) => {
            assert_eq!(v.len(), 3);
            assert_eq!(v[0].data, trips[1]);
            assert_eq!(v[1].data, trips[2]);
            assert_eq!(v[2].data, trips[3]);
        }
    }
}


#[test]
pub fn persists_and_reads_an_entry() {
    let series_remover = SeriesFileCleanup::new("var/persists_and_reads_an_entry.json");
    let trips = mk_trips();

    {
        let mut ts: Series<BikeTrip> = emseries::Series::open("var/persists_and_reads_an_entry").expect("expect the time series to open correctly");

        ts.put(trips[0].clone()).expect("expect a successful put");
        ts.put(trips[1].clone()).expect("expect a successful put");
        ts.put(trips[2].clone()).expect("expect a successful put");
        ts.put(trips[3].clone()).expect("expect a successful put");
        ts.put(trips[4].clone()).expect("expect a successful put");
    }

    {
        let mut ts: Series<BikeTrip> = emseries::Series::open("var/persists_and_reads_an_entry").expect("expect the time series to open correctly");
        match ts.search(time_range(Utc.ymd(2011, 10, 31).and_hms(0, 0, 0), true,
                                   Utc.ymd(2011, 11, 04).and_hms(0, 0, 0), true)) {
            Err(err) => assert!(false, err),
            Ok(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v[0].data, trips[1]);
                assert_eq!(v[1].data, trips[2]);
                assert_eq!(v[2].data, trips[3]);
            }
        }
    }
}


#[test]
pub fn reads_existing_file() {
    //let ts = emseries::Series::open("var/fixture-series");
    unimplemented!();
}

#[test]
pub fn can_write_to_existing_file() {
    //let ts = emseries::Series::open("var/fixture-series-2");
    unimplemented!();
}

#[test]
pub fn can_overwrite_existing_entry() {
    unimplemented!();
}

