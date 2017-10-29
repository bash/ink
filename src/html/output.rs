use super::builders::Event;
use std::fmt;

#[derive(Debug)]
pub struct Output<'a> {
    events: Vec<Event<'a>>,
}

impl<'a> fmt::Display for Output<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for event in self.events {
            write!(f, "{}", event)?;
        }

        Ok(())
    }
}