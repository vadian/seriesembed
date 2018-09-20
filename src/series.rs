extern crate uuid;

use std::fs::File;
use std::io::{ BufWriter };
use self::uuid::{ Uuid };

use criteria::{ Criteria };
use types::{ Error, Record, Recordable };

pub struct Series<T: Clone + Recordable> {
    path: String,
    writer: BufWriter<File>,
    records: Vec<Record<T>>,
}


impl <T> Series<T> 
    where T: Clone + Recordable
{
    pub fn open(path: &str) -> Result<Series<T>, Error> {
        let mut fullpath = String::from(path);
        fullpath.push_str(".json");
        let f = File::create(&fullpath).map_err(Error::IOError)?;
        let writer = BufWriter::new(f);
        
        Ok(Series{
            path: String::from(path),
            writer,
            records: Vec::new(),
        })
    }

    pub fn put(&mut self, entry: T) -> Result<Uuid, Error> {
        let rec = Record::new(entry);
        self.records.push(rec.clone());
        Ok(rec.id)
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

    pub fn get(&self, uuid: Uuid) -> Result<Option<Record<T>>, Error> {
        let mut matches: Vec<&Record<T>> = self.records.iter().filter(|r| r.id == uuid).collect();
        let val: Option<&Record<T>> = matches.pop();
        Ok(val.cloned())
    }
    
    pub fn remove(&self, uuid: Uuid) -> Result<(), Error> {
        unimplemented!()
    }
}


