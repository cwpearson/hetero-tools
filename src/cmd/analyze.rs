extern crate heteroprof;

use std::fs;
use std::io::BufReader;
use std::time::SystemTime;
use std::cmp::Ordering;

// use self::heteroprof::{decode_document, Document};

pub fn run(path: &str) {
    let start = SystemTime::now();
    let metadata = fs::metadata(path);
    let file = fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let doc = heteroprof::decode_document(&mut reader).unwrap();
    eprintln!("{} computes", doc.computes().len());
    eprintln!("{} transfers", doc.transfers().len());

    // print some info about execution
    let sz = metadata.unwrap().len();
    let dur = start.elapsed().unwrap();
    let secs = (dur.as_secs() as f64) + (dur.subsec_nanos() as f64) / 1e9;
    eprintln!("{} MB", sz / 1024 / 1024);
    eprintln!("{}s elapsed", secs);
    eprintln!("{}MB/s", sz as f64 / secs / 1024 as f64 / 1024 as f64);

    doc.computes().sort_by(|a, b| -> Ordering {
        return a.cmp_start(b);
    });
}
