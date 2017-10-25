extern crate ink;

use ink::BlockParser;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("demo.txt").unwrap();
    let mut reader = BufReader::new(&file);
    let parser = BlockParser::new(reader.lines());

    for block in parser {
        println!("{:?}", block);
    }
}