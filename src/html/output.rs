use super::builders::Event;
use std::fmt;

#[derive(Debug)]
pub struct Output {
    events: Vec<Event>,
}

impl Output {
    pub(crate) fn new(events: Vec<Event>) -> Self {
        Output { events }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for event in &self.events {
            write!(f, "{}", event)?;
        }

        Ok(())
    }
}
