extern crate squid;

use squid::Parser;
use squid::html::Renderer;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("examples/demo.sq").unwrap();
    let reader = BufReader::new(&file);
    let parser = Parser::new(reader.lines());
    let renderer = Renderer::new(parser);

    for block in renderer.take(6) {
        println!("{}", block.unwrap());
    }
}
