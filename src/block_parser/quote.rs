use super::TextAccumulator;
use super::{BlockParserInner, InspectOutput, TakeOutput};
use super::super::ast::Block;
use super::super::tokens::LineType;
use super::super::block_tokenizer::parse_quote;

#[derive(Debug)]
pub struct QuoteParser;

impl QuoteParser {
    pub fn new() -> Self {
        QuoteParser {}
    }
}

impl BlockParserInner for QuoteParser {
    fn inspect_line(&self, line_type: LineType, _: &str) -> InspectOutput {
        match line_type {
            LineType::Quote => InspectOutput::Accept,
            _ => InspectOutput::Reject,
        }
    }

    fn take_line(&mut self, line_type: LineType, line: &str) -> TakeOutput {
        self.inspect_line(line_type, line)
            .map_to_take(|| TakeOutput::Accepted)
    }

    fn process(&self, lines: Vec<(LineType, String)>) -> Block {
        let mut accumulator = TextAccumulator::new();

        for (_, line) in lines {
            accumulator.add(parse_quote(&line));
        }

        Block::Quote(accumulator.consume())
    }
}
