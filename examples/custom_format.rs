extern crate squid;

use squid::Parser;
use squid::ast::{Block, HeadingLevel};
use squid::html::{Format, Renderer};
use squid::html::builders::Builder;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct CustomFormat;

impl Format for CustomFormat {
    fn heading(&self, builder: &mut Builder, level: HeadingLevel, content: String) {
        let level_str = match level {
            HeadingLevel::Level2 => "2",
            HeadingLevel::Level3 => "3",
            _ => "1",
        };

        builder
            .tag_start("div")
            .add_attr("class", format!("heading-level-{}", level_str))
            .finish()
            .text(content)
            .tag_end("div");
    }
}

fn main() {
    let file = File::open("examples/demo.sq").unwrap();
    let reader = BufReader::new(&file);
    let parser = Parser::new(reader.lines());
    let renderer = Renderer::with_format(
        CustomFormat,
        parser.filter(|block| match *block {
            Ok(Block::Quote(..)) |
            Ok(Block::Paragraph(..)) => true,
            _ => false,
        }),
    );

    for block in renderer {
        println!("{}", block.unwrap());
    }
}
