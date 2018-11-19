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
use serde::ser::{ Serialize, Serializer };
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

#[test]
pub fn can_add_and_retrieve_entries() {
    let _series_remover = SeriesFileCleanup::new("var/can_add_and_retrieve_entries.json");
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_add_and_retrieve_entries.json").expect("expect the time series to open correctly");
    let uuid = ts.put(trips[0].clone()).expect("expect a successful put");
    let record_res = ts.get(&uuid);

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
    let _series_remover = SeriesFileCleanup::new("var/can_search_for_an_entry_with_exact_time.json");
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_search_for_an_entry_with_exact_time.json").expect("expect the time series to open correctly");
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
    let _series_remover = SeriesFileCleanup::new("var/can_get_entries_in_time_range.json");
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_get_entries_in_time_range.json").expect("expect the time series to open correctly");
    ts.put(trips[0].clone()).expect("expect a successful put");
    ts.put(trips[1].clone()).expect("expect a successful put");
    ts.put(trips[2].clone()).expect("expect a successful put");
    ts.put(trips[3].clone()).expect("expect a successful put");
    ts.put(trips[4].clone()).expect("expect a successful put");

    match ts.search_sorted(
        time_range(Utc.ymd(2011, 10, 31).and_hms(0, 0, 0), true,
                   Utc.ymd(2011, 11, 04).and_hms(0, 0, 0), true),
        |l, r| l.timestamp().cmp(&r.timestamp())) {
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
    let _series_remover = SeriesFileCleanup::new("var/persists_and_reads_an_entry.json");
    let trips = mk_trips();

    {
        let mut ts: Series<BikeTrip> = emseries::Series::open("var/persists_and_reads_an_entry.json").expect("expect the time series to open correctly");

        ts.put(trips[0].clone()).expect("expect a successful put");
        ts.put(trips[1].clone()).expect("expect a successful put");
        ts.put(trips[2].clone()).expect("expect a successful put");
        ts.put(trips[3].clone()).expect("expect a successful put");
        ts.put(trips[4].clone()).expect("expect a successful put");
    }

    {
        let ts: Series<BikeTrip> = emseries::Series::open("var/persists_and_reads_an_entry.json").expect("expect the time series to open correctly");
        match ts.search_sorted(
            time_range(Utc.ymd(2011, 10, 31).and_hms(0, 0, 0), true,
                       Utc.ymd(2011, 11, 04).and_hms(0, 0, 0), true),
            |l, r| l.timestamp().cmp(&r.timestamp())) {
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
pub fn can_write_to_existing_file() {
    let _series_remover = SeriesFileCleanup::new("var/can_write_to_existing_file.json");
    let trips = mk_trips();

    {
        let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_write_to_existing_file.json").expect("expect the time series to open correctly");

        ts.put(trips[0].clone()).expect("expect a successful put");
        ts.put(trips[1].clone()).expect("expect a successful put");
        ts.put(trips[2].clone()).expect("expect a successful put");
    }

    {
        let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_write_to_existing_file.json").expect("expect the time series to open correctly");
        match ts.search_sorted(
            time_range(Utc.ymd(2011, 10, 31).and_hms(0, 0, 0), true,
                       Utc.ymd(2011, 11, 04).and_hms(0, 0, 0), true),
            |l, r| l.timestamp().cmp(&r.timestamp())) {
            Err(err) => assert!(false, err),
            Ok(v) => {
                assert_eq!(v.len(), 2);
                assert_eq!(v[0].data, trips[1]);
                assert_eq!(v[1].data, trips[2]);
                ts.put(trips[3].clone()).expect("expect a successful put");
                ts.put(trips[4].clone()).expect("expect a successful put");
            }
        }
    }

    {
        let ts: Series<BikeTrip> = emseries::Series::open("var/can_write_to_existing_file.json").expect("expect the time series to open correctly");
        match ts.search_sorted(
            time_range(Utc.ymd(2011, 10, 31).and_hms(0, 0, 0), true,
                       Utc.ymd(2011, 11, 05).and_hms(0, 0, 0), true),
                |l, r| l.timestamp().cmp(&r.timestamp())) {
            Err(err) => assert!(false, err),
            Ok(v) => {
                assert_eq!(v.len(), 4);
                assert_eq!(v[0].data, trips[1]);
                assert_eq!(v[1].data, trips[2]);
                assert_eq!(v[2].data, trips[3]);
                assert_eq!(v[3].data, trips[4]);
            }
        }
    }
}

#[test]
pub fn can_overwrite_existing_entry() {
    let _series_remover = SeriesFileCleanup::new("var/can_overwrite_existing_entry.json");
    let trips = mk_trips();

    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_overwrite_existing_entry.json").expect("expect the time series to open correctly");

    ts.put(trips[0].clone()).expect("expect a successful put");
    ts.put(trips[1].clone()).expect("expect a successful put");
    let trip_id = ts.put(trips[2].clone()).expect("expect a successful put");

    match ts.get(&trip_id) {
        Err(err) => assert!(false, err),
        Ok(None) => assert!(false, "record not found"),
        Ok(Some(mut trip)) => {
            trip.data.distance = Distance(50000.0 * M);
            ts.update(trip).expect("expect record to update");
        }
    };

    match ts.get(&trip_id) {
        Err(err) => assert!(false, err),
        Ok(None) => assert!(false, "record not found"),
        Ok(Some(trip)) => {
            assert_eq!(trip.data.datetime, Utc.ymd(2011, 11, 02).and_hms(0, 0, 0));
            assert_eq!(trip.data.distance, Distance(50000.0 * M));
            assert_eq!(trip.data.duration, Duration(7020.0 * S));
            assert_eq!(trip.data.comments, String::from("Do Some Distance!"));
        }
    }
}

#[test]
pub fn record_overwrites_get_persisted() {
    let _series_remover = SeriesFileCleanup::new("var/record_overwrites_get_persisted.json");
    let trips = mk_trips();

    {
        let mut ts: Series<BikeTrip> = emseries::Series::open("var/record_overwrites_get_persisted.json").expect("expect the time series to open correctly");

        ts.put(trips[0].clone()).expect("expect a successful put");
        ts.put(trips[1].clone()).expect("expect a successful put");
        let trip_id = ts.put(trips[2].clone()).expect("expect a successful put");

        match ts.get(&trip_id) {
            Err(err) => assert!(false, err),
            Ok(None) => assert!(false, "record not found"),
            Ok(Some(mut trip)) => {
                trip.data.distance = Distance(50000.0 * M);
                ts.update(trip).expect("expect record to update");
            }
        };
    }

    {
        let ts: Series<BikeTrip> = emseries::Series::open("var/record_overwrites_get_persisted.json").expect("expect the time series to open correctly");

        match ts.all_records() {
            Err(err) => assert!(false, err),
            Ok(trips) => { assert_eq!(trips.len(), 3) },
        }

        match ts.search(exact_time(Utc.ymd(2011, 11, 02).and_hms(0, 0, 0))) {
            Err(err) => assert!(false, err),
            Ok(trips) => {
                assert_eq!(trips.len(), 1);
                assert_eq!(trips[0].data.datetime, Utc.ymd(2011, 11, 02).and_hms(0, 0, 0));
                assert_eq!(trips[0].data.distance, Distance(50000.0 * M));
                assert_eq!(trips[0].data.duration, Duration(7020.0 * S));
                assert_eq!(trips[0].data.comments, String::from("Do Some Distance!"));
            },
        }
    }
}

