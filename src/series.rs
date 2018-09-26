extern crate serde;
extern crate serde_json;
extern crate uuid;

use self::serde::de::DeserializeOwned;
use self::serde::ser::Serialize;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{ BufRead, BufReader, LineWriter, Write };

use criteria::{ Criteria };
use types::{ Error, Record, Recordable, UniqueId };

pub struct Series<T: Clone + Recordable + Serialize> {
    path: String,
    writer: LineWriter<File>,
    records: HashMap<UniqueId, Record<T>>,
}


impl <'de, T> Series<T> 
    where T: Clone + Recordable + DeserializeOwned + Serialize
{
    pub fn open(path: &str) -> Result<Series<T>, Error> {
        let mut fullpath = String::from(path);
        fullpath.push_str(".json");
        let f = OpenOptions::new().read(true)
                                  .append(true)
                                  .create(true)
                                  .open(&fullpath)
                                  .map_err(Error::IOError)?;

        let records = Series::load_file(&f)?;

        let writer = LineWriter::new(f);
        
        Ok(Series{
            path: String::from(path),
            writer,
            records,
        })
    }

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
        };
        Ok(records)
    }

    fn parse_line(line: &str) -> Result<Record<T>, Error> {
        serde_json::from_str(&line)
            .map_err(Error::DeserializationError)
    }

    pub fn put(&mut self, entry: T) -> Result<UniqueId, Error> {
        let record = Record::new(entry);
        let rec_id = record.id.clone();
        self.update(record).and_then(|()| Ok(rec_id))
    }

    pub fn update(&mut self, record: Record<T>) -> Result<(), Error> {
        self.records.insert(record.id.clone(), record.clone());
        let write_res = match serde_json::to_string(&record) {
            Ok(rec_str) => {
                self.writer.write_fmt(format_args!("{}\n", rec_str.as_str())).map_err(Error::IOError)
            },
            Err(err) => Err(Error::SerializationError(err)),
        };

        match write_res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn search<C>(&self, criteria: C) -> Result<Vec<Record<T>>, Error>
        where C: Criteria {
        let results: Vec<Record<T>> =
            self.records
                .iter()
                .filter(|&tr| criteria.apply(tr.1))
                .map(|tr| tr.1.clone())
                .collect();
        Ok(results)
    }

    pub fn search_sorted<C, CMP>(&self, criteria: C, compare: CMP) -> Result<Vec<Record<T>>, Error>
        where C: Criteria,
              CMP: FnMut(&Record<T>, &Record<T>) -> Ordering
    {
        match self.search(criteria) {
            Ok(mut records) => {
                records.sort_by(compare);
                Ok(records)
            },
            Err(err) => Err(err),
        }
    }

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


