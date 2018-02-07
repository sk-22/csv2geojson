extern crate csv;
extern crate geojson;

use std::error::Error;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::process;
use geojson::{Feature, GeoJson, Geometry, Value};

fn main() {    
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}


fn run() -> Result<(),  Box<Error>> {
    let file_path = get_first_arg()?;

    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) =>{
                let mut r = record.iter();
                let lat : f64 = r.next().unwrap().to_string().parse().unwrap();
                let lon : f64 = r.next().unwrap().to_string().parse().unwrap();
                let mut v: Vec<f64> = Vec::new();
                v.push(lat);
                v.push(lon);
                let geometry = Geometry::new(
                    Value::Point(v)
                );

                let geojson = GeoJson::Feature(Feature {
                    bbox: None,
                    geometry: Some(geometry),
                    id: None,
                    properties: None,
                    foreign_members: None
                });
                let geojson_string = geojson.to_string();
                println!("{:?}", geojson_string);
            }
        }
    }
    Ok(())
}