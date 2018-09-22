extern crate chrono;
extern crate dimensioned;
extern crate emseries;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use chrono::prelude::*;
use dimensioned::si::{ M, Meter, S, Second };
use std::fs;
use std::ops;
use serde::ser::{ Serialize, Serializer, SerializeStruct };
//use uuid::Uuid;

use emseries::*;

#[derive(Clone, Debug, PartialEq)]
struct BikeTrip {
    datetime: DateTime<Utc>,
    distance: Meter<f64>,
    duration: Second<f64>,
    comments: String,
}

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
            distance: 58741.055 * M,
            duration: 11040.0 * S,
            comments: String::from("long time ago"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 10, 31).and_hms(0, 0, 0),
            distance: 17702.0 * M,
            duration: 2880.0 * S,
            comments: String::from("day 2"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 11, 02).and_hms(0, 0, 0),
            distance: 41842.945 * M,
            duration: 7020.0 * S,
            comments: String::from("Do Some Distance!"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 11, 04).and_hms(0, 0, 0),
            distance: 34600.895 * M,
            duration: 5580.0 * S,
            comments: String::from("I did a lot of distance back then"),
        },
        BikeTrip{
            datetime: Utc.ymd(2011, 11, 05).and_hms(0, 0, 0),
            distance: 6437.376 * M,
            duration: 960.0 * S,
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
pub fn can_add_and_retrieve_an_entry() {
    let series_remover = SeriesFileCleanup::new("var/can_add_and_retrieve_an_entry.json");
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/can_add_and_retrieve_an_entry").expect("expect the time series to open correctly");
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
            assert_eq!(tr.data.duration, 11040.0 * S);
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

    unimplemented!();

    /*
    match uuid {
        Err(err) => assert!(false, err),
        Ok(uuid_) => {
            /*
            let record_res = ts.search(exact_time(Utc.ymd(2011, 10, 29).and_hms(0, 0, 0)));
            match record_res {
                Ok(tr) => assert_eq!(tr.len(), 1),
                Err(err) => assert!(false, err),
            }
            */

            let record_res_2: Result<Option<Record<BikeTrip>>, Error> = ts.get(uuid_);
            match record_res_2 {
                Err(err) => assert!(false, err),
                Ok(None) => assert!(false, "There should have been a value here"),
                Ok(Some(tr)) => {
                    assert_eq!(tr.id, uuid_);
                    assert_eq!(tr.timestamp(), Utc.ymd(2011, 10, 29).and_hms(0, 0, 0));
                    assert_eq!(tr.data.duration, 11040.0 * S);
                    assert_eq!(tr.data.comments, String::from("long time ago"));
                }
            }
        }
    }
    */
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

