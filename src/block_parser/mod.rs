use super::tokens::LineType;
use super::ast::{Block, Inline, Text};

mod paragraph;
mod quote;

use self::paragraph::ParagraphProcessor;
use self::quote::QuoteProcessor;

#[derive(Debug)]
pub struct TextAccumulator {
    buffer: String,
}

pub trait BlockProcessor {
    fn can_process(&self, line_type: LineType) -> bool;
    fn process_line(&mut self, line_type: LineType, line: String);
    fn consume(self) -> Block;
}

#[derive(Debug)]
pub enum BlockParser {
    Paragraph(ParagraphProcessor),
    Quote(QuoteProcessor),
}

impl BlockParser {
    pub fn processor(&self) -> &BlockProcessor {
        match *self {
            BlockParser::Paragraph(ref processor) => processor,
            BlockParser::Quote(ref processor) => processor,
        }
    }

    pub fn processor_mut(&mut self) -> &mut BlockProcessor {
        match *self {
            BlockParser::Paragraph(ref mut processor) => processor,
            BlockParser::Quote(ref mut processor) => processor,
        }
    }

    pub fn consume(self) -> Block {
        match self {
            BlockParser::Paragraph(processor) => processor.consume(),
            BlockParser::Quote(processor) => processor.consume(),
        }
    }

    pub fn for_line_type(line_type: LineType) -> Self {
        match line_type {
            LineType::Blank => panic!("no processor can handle blank lines"),
            LineType::Text => BlockParser::Paragraph(ParagraphProcessor::new()),
            LineType::Quote => BlockParser::Quote(QuoteProcessor::new()),
            LineType::Heading1 |
            LineType::Heading2 |
            LineType::Heading3 => {
                panic!("no processor can handle headings");
            }
            _ => unimplemented!(),
        }
    }
}

impl TextAccumulator {
    pub fn new() -> Self {
        TextAccumulator { buffer: String::new() }
    }

    ///
    /// Adds a new line to the current accumulated text.
    ///
    pub fn add(&mut self, line: String) {
        if !self.buffer.is_empty() {
            self.buffer.push_str(" ");
        }

        self.buffer.push_str(line.trim());
    }

    pub fn consume(self) -> Text {
        vec![Inline::Chunk(self.buffer)]
    }
}
