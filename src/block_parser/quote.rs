use super::TextAccumulator;
use super::BlockProcessor;
use super::super::ast::Block;
use super::super::tokens::LineType;
use super::super::constants;

#[derive(Debug)]
pub struct QuoteProcessor {
    accumulator: TextAccumulator,
}

impl QuoteProcessor {
    pub fn new() -> Self {
        QuoteProcessor {
            accumulator: TextAccumulator::new(),
        }
    }
}

impl BlockProcessor for QuoteProcessor {
    fn can_process(&self, line_type: LineType) -> bool {
        matches!(line_type, LineType::Quote)
    }

    fn process_line(&mut self, _: LineType, line: String) {
        let prefix_len = constants::QUOTE_TOKEN.len();
        let line = line.chars().skip(prefix_len).collect();

        self.accumulator.add(line);
    }

    fn consume(self) -> Block {
        Block::Quote(self.accumulator.consume())
    }
}
