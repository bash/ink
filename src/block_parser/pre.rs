use super::{BlockParserInner, InspectOutput, TakeOutput};
use super::super::ast::{Block, Decorator};
use super::super::tokens::LineType;
use super::super::block_tokenizer::parse_decorator;

#[derive(Debug, Copy, Clone)]
enum State {
    Initial,
    Decorator,
    InsideBlock,
    Finished,
}

#[derive(Debug, Default)]
pub struct PreParser {
    state: State,
}

impl Default for State {
    fn default() -> Self {
        State::Initial
    }
}

impl PreParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl BlockParserInner for PreParser {
    fn inspect_line(&self, line_type: LineType, _: &str) -> InspectOutput {
        match self.state {
            State::Initial => accept!(line_type, LineType::Decorator | LineType::Divider),
            State::InsideBlock => InspectOutput::Accept,
            State::Decorator => accept!(line_type, LineType::Divider),
            State::Finished => InspectOutput::Reject,
        }
    }

    fn take_line(&mut self, line_type: LineType, line: &str) -> TakeOutput {
        self.inspect_line(line_type, line).map_to_take(|| {
            self.state = match (self.state, line_type) {
                (State::Initial, LineType::Decorator) => State::Decorator,
                (State::Initial, LineType::Divider) | (State::Decorator, LineType::Divider) => {
                    State::InsideBlock
                }
                (State::InsideBlock, LineType::Divider) => State::Finished,
                _ => {
                    return TakeOutput::Fallback;
                }
            };

            TakeOutput::Accepted
        })
    }

    fn process(&self, lines: Vec<(LineType, String)>) -> Block {
        let mut decorator: Option<Decorator> = None;
        let mut pre: Vec<String> = Vec::new();

        for (line_type, line) in lines {
            match line_type {
                LineType::Decorator => decorator = parse_decorator(&line),
                LineType::Divider => continue,
                _ => pre.push(line),
            };
        }

        Block::Preformatted(decorator, pre)
    }
}
