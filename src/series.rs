extern crate serde;
extern crate serde_json;
extern crate uuid;

use self::serde::ser::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{ BufRead, BufReader, LineWriter, Write };

use criteria::{ Criteria };
use types::{ Error, Record, Recordable, UniqueId };

pub struct Series<T: Clone + Recordable + Serialize> {
    path: String,
    writer: LineWriter<File>,
    records: Vec<Record<T>>,
}


impl <T> Series<T> 
    where T: Clone + Recordable + Serialize
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

    fn load_file(f: &File) -> Result<Vec<Record<T>>, Error> {
        let reader = BufReader::new(f);
        for line in reader.lines() {
            println!("[line] {:?}", line);
        }
        Ok(Vec::new())
    }

    pub fn put(&mut self, entry: T) -> Result<UniqueId, Error> {
        let rec = Record::new(entry);
        self.records.push(rec.clone());
        let write_res = match serde_json::to_string(&rec) {
            Ok(rec_str) => {
                println!("[put] {}", rec_str.as_str());
                self.writer.write_fmt(format_args!("{}\n", rec_str.as_str())).map_err(Error::IOError)
            },
            Err(err) => Err(Error::SerializationError(err)),
        };

        match write_res {
            Ok(_) => Ok(rec.id),
            Err(err) => Err(err),
        }
    }

    pub fn search<C>(&self, criteria: C) -> Result<Vec<Record<T>>, Error>
        where C: Criteria {
        let results: Vec<Record<T>> =
            self.records
                .iter()
                .filter(|&tr| criteria.apply(tr))
                .map(|tr| tr.clone())
                .collect();
        Ok(results)
    }

    pub fn get(&self, uuid: UniqueId) -> Result<Option<Record<T>>, Error> {
        let mut matches: Vec<&Record<T>> = self.records.iter().filter(|r| r.id == uuid).collect();
        let val: Option<&Record<T>> = matches.pop();
        Ok(val.cloned())
    }
    
    pub fn remove(&self, uuid: UniqueId) -> Result<(), Error> {
        unimplemented!()
    }
}


