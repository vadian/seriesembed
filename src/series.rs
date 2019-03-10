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
                    match Series::parse_line(&line_) {
                        Ok(record) => records.insert(record.id.clone(), record.clone()),
                        Err(err) => return Err(err),
                    };
                }
                Err(err) => return Err(Error::IOError(err)),
            }
        }
        Ok(records)
    }

    /* TODO: make this private after merging in the serialization tests in */
    pub fn parse_line(line: &str) -> Result<Record<T>, Error> {
        serde_json::from_str(&line).map_err(Error::DeserializationError)
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
