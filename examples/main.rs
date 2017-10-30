extern crate squid;

use squid::BlockParser;
use squid::html::Renderer;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("examples/demo.sq").unwrap();
    let reader = BufReader::new(&file);
    let parser = BlockParser::new(reader.lines());
    let renderer = Renderer::new(parser);

    for block in renderer.take(3) {
        println!("{}", block.unwrap());
    }
}
