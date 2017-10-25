extern crate squid;

use squid::BlockParser;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("demo.sq").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let parser = BlockParser::new(input);

    for block in parser {
        println!("{:?}", block);
    }
}
