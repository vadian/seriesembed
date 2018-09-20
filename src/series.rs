extern crate uuid;

use std::fs::File;
use std::io::{ BufWriter };
use self::uuid::{ Uuid };

use criteria::{ Criteria };
use types::{ Error, Record };

pub struct Series<T> {
    path: String,
    writer: BufWriter<File>,
    records: Vec<T>,
}


impl <T> Series<T> 
    where T: Record
{
    pub fn new(path: &str) -> Result<Series<T>, Error> {
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

    pub fn put(& mut self, entry: T) -> Result<Uuid, Error> {
        unimplemented!()
    }

    pub fn search<C>(&self, criteria: C) -> Result<Vec<T>, Error>
        where C: Criteria {
        unimplemented!()
    }

    pub fn get(&self, uuid: Uuid) -> Result<Option<T>, Error> {
        unimplemented!()
    }
    
    pub fn remove(&self, uuid: Uuid) -> Result<(), Error> {
        unimplemented!()
    }
}


