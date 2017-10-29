extern crate squid;

use squid::BlockParser;
use squid::html::Generator;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("examples/demo.sq").unwrap();
    let reader = BufReader::new(&file);
    let parser = BlockParser::new(reader.lines());
    let generator = Generator::new(parser);

    for block in generator.take(3) {
        println!("{}", block.unwrap());
    }
}
