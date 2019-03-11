extern crate serde;
extern crate serde_json;
extern crate uuid;

use self::serde::de::DeserializeOwned;
use self::serde::ser::Serialize;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, LineWriter, Write};

use criteria::Criteria;
use types::{Error, Record, Recordable, UniqueId};

/// An open time series database.
///
/// Any given database can store only one data type, T. The data type must be determined when the
/// database is opened.
pub struct Series<T: Clone + Recordable + Serialize> {
    //path: String,
    writer: LineWriter<File>,
    records: HashMap<UniqueId, Record<T>>,
}


impl<'de, T> Series<T>
where
    T: Clone + Recordable + DeserializeOwned + Serialize,
{
    /// Open a time series database at the specified path. `path` is the full path and filename for
    /// the database.
    pub fn open(path: &str) -> Result<Series<T>, Error> {
        let f = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)
            .map_err(Error::IOError)?;

        let records = Series::load_file(&f)?;

        let writer = LineWriter::new(f);

        Ok(Series {
            //path: String::from(path),
            writer,
            records,
        })
    }

    /// Load a file and return all of the records in it.
    fn load_file(f: &File) -> Result<HashMap<UniqueId, Record<T>>, Error> {
        let mut records: HashMap<UniqueId, Record<T>> = HashMap::new();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            match line {
                Ok(line_) => {
                    match parse_line(&line_) {
                        Ok(record) => records.insert(record.id.clone(), record.clone()),
                        Err(err) => return Err(err),
                    };
                }
                Err(err) => return Err(Error::IOError(err)),
            }
        }
        Ok(records)
    }

    /// Put a new record into the database. A unique id will be assigned to the record and
    /// returned.
    pub fn put(&mut self, entry: T) -> Result<UniqueId, Error> {
        let record = Record::new(entry);
        let rec_id = record.id.clone();
        self.update(record).and_then(|()| Ok(rec_id))
    }

    /// Update an existing record. The `UniqueId` of the record passed into this function must match
    /// the `UniqueId` of a record already in the database.
    pub fn update(&mut self, record: Record<T>) -> Result<(), Error> {
        self.records.insert(record.id.clone(), record.clone());
        let write_res = match serde_json::to_string(&record) {
            Ok(rec_str) => {
                self.writer
                    .write_fmt(format_args!("{}\n", rec_str.as_str()))
                    .map_err(Error::IOError)
            }
            Err(err) => Err(Error::SerializationError(err)),
        };

        match write_res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Get all of the records in the database.
    pub fn all_records(&self) -> Result<Vec<Record<T>>, Error> {
        let results = self.records.iter().map(|tr| tr.1.clone()).collect();
        Ok(results)
    }

    /* Commented out because I just cannot figure out the type signature
    pub fn records(&self) -> Result<Map<Iterator<Item=Record<T>>, FnMut(Record<T>) -> Record<T>>, Error> {
    pub fn records(&self) -> std::result::Result<Map<hash_map::Iter<'_, UniqueId, Record<T>>, Fn(Record<T>) -> Record<T>>, Error> {
        let results = self.records.iter().map(|tr| tr.1.clone());
        Ok(results)
    }
    */

    /*  The point of having Search is so that a lot of internal optimizations can happen once the
     *  data sets start getting large. */
    /// Perform a search on the records in a database, based on the given criteria.
    pub fn search<C>(&self, criteria: C) -> Result<Vec<Record<T>>, Error>
    where
        C: Criteria,
    {
        let results: Vec<Record<T>> = self.records
            .iter()
            .filter(|&tr| criteria.apply(tr.1))
            .map(|tr| tr.1.clone())
            .collect();
        Ok(results)
    }

    /// Perform a search and sort the resulting records based on the comparison.
    pub fn search_sorted<C, CMP>(&self, criteria: C, compare: CMP) -> Result<Vec<Record<T>>, Error>
    where
        C: Criteria,
        CMP: FnMut(&Record<T>, &Record<T>) -> Ordering,
    {
        match self.search(criteria) {
            Ok(mut records) => {
                records.sort_by(compare);
                Ok(records)
            }
            Err(err) => Err(err),
        }
    }

    /// Get an exact record from the database based on unique id.
    pub fn get(&self, uuid: &UniqueId) -> Result<Option<Record<T>>, Error> {
        let val = self.records.get(uuid);
        Ok(val.cloned())
    }

    /*
    pub fn remove(&self, uuid: UniqueId) -> Result<(), Error> {
        unimplemented!()
    }
    */
}

fn parse_line<T>(line: &str) -> Result<Record<T>, Error>
where
    T: Clone + Recordable + DeserializeOwned + Serialize,
{
    serde_json::from_str(&line).map_err(Error::DeserializationError)
}


#[cfg(test)]
mod tests {
    extern crate chrono;
    extern crate dimensioned;

    use self::chrono::prelude::*;
    use self::dimensioned::si::{M, Meter, S, Second, KG, Kilogram};
    use std::fs;
    use std::ops;

    use super::*;
    use criteria::*;

    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct Distance(Meter<f64>);


    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    struct Duration(Second<f64>);

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
        fn tags(&self) -> Vec<String> {
            Vec::new()
        }
        fn values(&self) -> Vec<String> {
            Vec::new()
        }
    }

    struct SeriesFileCleanup(String);

    impl SeriesFileCleanup {
        fn new(path: &str) -> SeriesFileCleanup {
            SeriesFileCleanup(String::from(path))
        }
    }

    impl ops::Drop for SeriesFileCleanup {
        fn drop(&mut self) {
            fs::remove_file(&self.0).expect("failed to remove time series file");
        }
    }

    fn mk_trips() -> [BikeTrip; 5] {
        [
            BikeTrip {
                datetime: Utc.ymd(2011, 10, 29).and_hms(0, 0, 0),
                distance: Distance(58741.055 * M),
                duration: Duration(11040.0 * S),
                comments: String::from("long time ago"),
            },
            BikeTrip {
                datetime: Utc.ymd(2011, 10, 31).and_hms(0, 0, 0),
                distance: Distance(17702.0 * M),
                duration: Duration(2880.0 * S),
                comments: String::from("day 2"),
            },
            BikeTrip {
                datetime: Utc.ymd(2011, 11, 02).and_hms(0, 0, 0),
                distance: Distance(41842.945 * M),
                duration: Duration(7020.0 * S),
                comments: String::from("Do Some Distance!"),
            },
            BikeTrip {
                datetime: Utc.ymd(2011, 11, 04).and_hms(0, 0, 0),
                distance: Distance(34600.895 * M),
                duration: Duration(5580.0 * S),
                comments: String::from("I did a lot of distance back then"),
            },
            BikeTrip {
                datetime: Utc.ymd(2011, 11, 05).and_hms(0, 0, 0),
                distance: Distance(6437.376 * M),
                duration: Duration(960.0 * S),
                comments: String::from("day 5"),
            },
        ]
    }

    #[test]
    pub fn can_add_and_retrieve_entries() {
        let _series_remover = SeriesFileCleanup::new("var/can_add_and_retrieve_entries.json");
        let trips = mk_trips();
        let mut ts: Series<BikeTrip> = Series::open("var/can_add_and_retrieve_entries.json")
            .expect("expect the time series to open correctly");
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
        let _series_remover =
            SeriesFileCleanup::new("var/can_search_for_an_entry_with_exact_time.json");
        let trips = mk_trips();
        let mut ts: Series<BikeTrip> = Series::open(
            "var/can_search_for_an_entry_with_exact_time.json",
        ).expect("expect the time series to open correctly");
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
        let mut ts: Series<BikeTrip> = Series::open("var/can_get_entries_in_time_range.json")
            .expect("expect the time series to open correctly");
        ts.put(trips[0].clone()).expect("expect a successful put");
        ts.put(trips[1].clone()).expect("expect a successful put");
        ts.put(trips[2].clone()).expect("expect a successful put");
        ts.put(trips[3].clone()).expect("expect a successful put");
        ts.put(trips[4].clone()).expect("expect a successful put");

        match ts.search_sorted(
            time_range(
                Utc.ymd(2011, 10, 31).and_hms(0, 0, 0),
                true,
                Utc.ymd(2011, 11, 04).and_hms(0, 0, 0),
                true,
            ),
            |l, r| l.timestamp().cmp(&r.timestamp()),
        ) {
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
            let mut ts: Series<BikeTrip> = Series::open("var/persists_and_reads_an_entry.json")
                .expect("expect the time series to open correctly");

            ts.put(trips[0].clone()).expect("expect a successful put");
            ts.put(trips[1].clone()).expect("expect a successful put");
            ts.put(trips[2].clone()).expect("expect a successful put");
            ts.put(trips[3].clone()).expect("expect a successful put");
            ts.put(trips[4].clone()).expect("expect a successful put");
        }

        {
            let ts: Series<BikeTrip> = Series::open("var/persists_and_reads_an_entry.json")
                .expect("expect the time series to open correctly");
            match ts.search_sorted(
                time_range(
                    Utc.ymd(2011, 10, 31).and_hms(0, 0, 0),
                    true,
                    Utc.ymd(2011, 11, 04).and_hms(0, 0, 0),
                    true,
                ),
                |l, r| l.timestamp().cmp(&r.timestamp()),
            ) {
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
            let mut ts: Series<BikeTrip> = Series::open("var/can_write_to_existing_file.json")
                .expect("expect the time series to open correctly");

            ts.put(trips[0].clone()).expect("expect a successful put");
            ts.put(trips[1].clone()).expect("expect a successful put");
            ts.put(trips[2].clone()).expect("expect a successful put");
        }

        {
            let mut ts: Series<BikeTrip> = Series::open("var/can_write_to_existing_file.json")
                .expect("expect the time series to open correctly");
            match ts.search_sorted(
                time_range(
                    Utc.ymd(2011, 10, 31).and_hms(0, 0, 0),
                    true,
                    Utc.ymd(2011, 11, 04).and_hms(0, 0, 0),
                    true,
                ),
                |l, r| l.timestamp().cmp(&r.timestamp()),
            ) {
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
            let ts: Series<BikeTrip> = Series::open("var/can_write_to_existing_file.json").expect(
                "expect the time series to open correctly",
            );
            match ts.search_sorted(
                time_range(
                    Utc.ymd(2011, 10, 31).and_hms(0, 0, 0),
                    true,
                    Utc.ymd(2011, 11, 05).and_hms(0, 0, 0),
                    true,
                ),
                |l, r| l.timestamp().cmp(&r.timestamp()),
            ) {
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

        let mut ts: Series<BikeTrip> = Series::open("var/can_overwrite_existing_entry.json")
            .expect("expect the time series to open correctly");

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
            let mut ts: Series<BikeTrip> = Series::open("var/record_overwrites_get_persisted.json")
                .expect("expect the time series to open correctly");

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
            let ts: Series<BikeTrip> = Series::open("var/record_overwrites_get_persisted.json")
                .expect("expect the time series to open correctly");

            match ts.all_records() {
                Err(err) => assert!(false, err),
                Ok(trips) => assert_eq!(trips.len(), 3),
            }

            match ts.search(exact_time(Utc.ymd(2011, 11, 02).and_hms(0, 0, 0))) {
                Err(err) => assert!(false, err),
                Ok(trips) => {
                    assert_eq!(trips.len(), 1);
                    assert_eq!(
                        trips[0].data.datetime,
                        Utc.ymd(2011, 11, 02).and_hms(0, 0, 0)
                    );
                    assert_eq!(trips[0].data.distance, Distance(50000.0 * M));
                    assert_eq!(trips[0].data.duration, Duration(7020.0 * S));
                    assert_eq!(trips[0].data.comments, String::from("Do Some Distance!"));
                }
            }
        }
    }


    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
    pub struct Weight(Kilogram<f64>);

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
        let rec: Result<Record<WeightRecord>, Error> = parse_line(WEIGHT_ENTRY);
        match rec {
            Err(err) => assert!(false, err),
            Ok(rec_) => {
                assert_eq!(
                    rec_.id,
                    UniqueId::from_str("3330c5b0-783f-4919-b2c4-8169c38f65ff").unwrap()
                );
                assert_eq!(rec_.data.weight, Weight(77.79109 * KG));
            }
        }
    }

    #[test]
    pub fn legacy_file_load() {
        let ts: Series<WeightRecord> =
            Series::open("fixtures/weight.json").expect("legacy series should open correctly");

        for record in ts.all_records().expect("all_records should never fail") {
            println!("[Record] {:?}", record);
        }

        let uid = UniqueId::from_str("3330c5b0-783f-4919-b2c4-8169c38f65ff")
            .expect("something is wrong with this ID");
        let rec = ts.get(&uid);
        match rec {
            Err(err) => assert!(false, err),
            Ok(None) => assert!(false, "no record found"),
            Ok(Some(rec)) => assert_eq!(rec.data.weight, Weight(77.79109 * KG)),
        }
    }

    #[test]
    pub fn serialization_output() {
        let date = Utc.ymd(2003, 11, 10).and_hms(6, 0, 0);
        let rec = WeightRecord {
            date,
            weight: Weight(77.0 * KG),
        };
        assert_eq!(
            serde_json::to_string(&rec).unwrap(),
            "{\"date\":\"2003-11-10T06:00:00Z\",\"weight\":77.0}"
        );
    }

}
