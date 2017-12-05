#![feature(link_args)]

#[cfg_attr(target_arch="wasm32", link_args = "--embed-file binary_data.dat@binary_data.dat")]
extern {}

extern crate byteorder;

use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::mem::size_of;
use byteorder::{LittleEndian, ReadBytesExt};

//use std::io::BufReader;

struct Row {
  px: f64,
  py: f64,
  pdx: f64,
  pdy: f64,
  val: f64,
}

fn main() {
    let filename = "./binary_data.dat";

    let mut f = File::open(filename);
    let mut f = match f {
      Ok(file)   => file,
      Err(error) => {
                      panic!("There was a problem opening the file: {:?}", error)
                    },
    };
    let len = match f.metadata() {
        Ok(v) => v.len() as usize,
        Err(_) => 0,
    };
    let rs = size_of::<Row>();
    let n_rows = len / rs;
    println!("Reading {} bytes from {} for {} rows\n", len, filename, n_rows);
    
    let mut row: Row;
    let mut rows: Vec<Row> = Vec::with_capacity(n_rows);

    let mut row_count = 0;

    while row_count < n_rows {
        rows.push(Row {
            val: f.read_f64::<LittleEndian>().unwrap(),
            pdx: f.read_f64::<LittleEndian>().unwrap(),
            pdy: f.read_f64::<LittleEndian>().unwrap(),
            px: f.read_f64::<LittleEndian>().unwrap(),
            py: f.read_f64::<LittleEndian>().unwrap(),
        });
        row_count += 1;
    }
}
