extern crate heteroprof;

use std::fs::File;
use std::io::BufReader;

use self::heteroprof::{decode_document, Document};

pub fn run(path: &str) {
    println!("analyzing!!!");

    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let doc = heteroprof::decode_document(&mut reader).unwrap();
    println!("{}", doc.computes().len());
}
