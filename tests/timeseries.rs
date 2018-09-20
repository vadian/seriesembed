extern crate chrono;
extern crate dimensioned;
extern crate emseries;
extern crate uuid;

use chrono::prelude::*;
use dimensioned::si::{ M, Meter, S, Second };
use uuid::Uuid;

use emseries::*;

#[derive(Clone, Debug, PartialEq)]
struct BikeTrip {
    datetime: DateTime<Utc>,
    distance: Meter<f64>,
    duration: Second<f64>,
    comments: String,
}

impl Recordable for BikeTrip {
    fn timestamp(&self) -> DateTime<Utc> {
        self.datetime
    }
    fn tags(&self) -> Vec<String> { Vec::new() }
    fn values(&self) -> Vec<String> { Vec::new() }
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


#[test]
pub fn can_add_and_retrieve_an_entry() {
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/series1").expect("expect the time series to open correctly");
    let uuid = ts.put(trips[0].clone()).expect("expect a successful put");
    let record_res = ts.get(uuid);

    ts.put(trips[1].clone());
    ts.put(trips[2].clone());
    ts.put(trips[3].clone());
    ts.put(trips[4].clone());

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
    unimplemented!()
}


#[test]
pub fn can_get_entries_in_time_range() {
    unimplemented!()
}


#[test]
/*
pub fn persists_and_reads_an_entry() {
    let trips = mk_trips();
    let mut ts: Series<BikeTrip> = emseries::Series::open("var/series1").expect("expect the time series to open correctly");

    let uuid = ts.put(trips[0].clone());
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

}
*/

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

