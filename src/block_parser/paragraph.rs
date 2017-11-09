use super::TextAccumulator;
use super::BlockProcessor;
use super::super::ast::Block;
use super::super::tokens::LineType;

#[derive(Debug)]
pub struct ParagraphProcessor {
    accumulator: TextAccumulator,
}

impl ParagraphProcessor {
    pub fn new() -> Self {
        ParagraphProcessor {
            accumulator: TextAccumulator::new(),
        }
    }
}

impl BlockProcessor for ParagraphProcessor {
    fn can_process(&self, line_type: LineType) -> bool {
        matches!(line_type, LineType::Text)
    }

    fn process_line(&mut self, _: LineType, line: String) {
        self.accumulator.add(line);
    }

    fn consume(self) -> Block {
        Block::Paragraph(self.accumulator.consume())
    }
}
