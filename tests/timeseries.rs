extern crate chrono;
extern crate dimensioned;
extern crate emseries;
extern crate uuid;

use chrono::prelude::*;
use dimensioned::si::{ S, Second };
use uuid::Uuid;

use emseries::*;

#[derive(Clone, Debug)]
struct BikeTrip {
    id: Option<Uuid>,
    datetime: DateTime<Utc>,
    duration: Second<f64>,
    comments: String,
}

impl Record for BikeTrip {
    fn id(&self) -> Option<Uuid> {
        self.id
    }
    fn set_id(&mut self, uuid: Uuid) {
        self.id = Some(uuid);
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.datetime
    }
    fn tags(&self) -> Vec<String> { Vec::new() }
    fn values(&self) -> Vec<String> { Vec::new() }
}

#[test]
pub fn persists_and_reads_an_entry() {
    let mut ts: Series<BikeTrip> = emseries::Series::new("var/series1").expect("expect the time series to open correctly");

    let trip = BikeTrip{
        id: None,
        datetime: Utc.ymd(2011, 10, 29).and_hms(0, 0, 0),
        duration: 11040.0 * S,
        comments: String::from("long time ago"),
    };

    let uuid = ts.put(trip);
    match uuid {
        Err(err) => assert!(false, err),
        Ok(uuid_) => {
            let record_res = ts.search(exact_time(Utc.ymd(2011, 10, 29).and_hms(0, 0, 0)));
            match record_res {
                Ok(tr) => assert_eq!(tr.len(), 1),
                Err(err) => assert!(false, err),
            }

            let record_res_2: Result<Option<BikeTrip>, Error> = ts.get(uuid_);
            match record_res_2 {
                Err(err) => assert!(false, err),
                Ok(None) => assert!(false, "There should have been a value here"),
                Ok(Some(tr)) => {
                    assert_eq!(tr.id(), Some(uuid_));
                    assert_eq!(tr.timestamp(), Utc.ymd(2011, 10, 29).and_hms(0, 0, 0));
                    assert_eq!(tr.duration, 11040.0 * S);
                    assert_eq!(tr.comments, String::from("long time ago"));
                }
            }
        }
    }

}

#[test]
pub fn reads_existing_file() {
    //let ts = emseries::Series::new("var/fixture-series");
    unimplemented!();
}

#[test]
pub fn can_write_to_existing_file() {
    //let ts = emseries::Series::new("var/fixture-series-2");
    unimplemented!();
}

#[test]
pub fn can_overwrite_existing_entry() {
    unimplemented!();
}

