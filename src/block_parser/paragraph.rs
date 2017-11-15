use super::TextAccumulator;
use super::{BlockParserInner, InspectOutput, TakeOutput};
use super::super::ast::Block;
use super::super::tokens::LineType;

#[derive(Debug)]
pub struct ParagraphParser;

impl ParagraphParser {
    pub fn new() -> Self {
        ParagraphParser {}
    }
}

impl BlockParserInner for ParagraphParser {
    fn inspect_line(&self, line_type: LineType, line: &str) -> InspectOutput {
        match line_type {
            LineType::Text => InspectOutput::Accept,
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
            accumulator.add(line);
        }

        Block::Paragraph(accumulator.consume())
    }
}
