use std::fmt;
use std::mem;
use std::borrow::Cow;
use super::escape::Escape;

#[derive(Debug)]
pub struct Builder<'a> {
    events: Vec<Event<'a>>,
}

#[derive(Debug)]
pub struct TagStartBuilder<'a, 'b: 'a> {
    name: Cow<'b, str>,
    attrs: Vec<Attribute<'b>>,
    builder: &'a mut Builder<'b>,
}

#[derive(Debug)]
pub struct Output<'a> {
    events: Vec<Event<'a>>,
}

type Attribute<'a> = (Cow<'a, str>, Cow<'a, str>);

// We're not exposing `Event` directly which allows us
// to make changes to the representation without breaking api compatiblity
// Consumers have to use the respective methods on a `Builder` instance instead.
#[derive(Debug)]
pub enum Event<'a> {
    TagStart {
        name: Cow<'a, str>,
        attrs: Vec<Attribute<'a>>,
    },
    Text { text: Cow<'a, str> },
    TagEnd { name: Cow<'a, str> },
}

fn format_events<'a>(f: &mut fmt::Formatter, events: &Vec<Event<'a>>) -> fmt::Result {
    for event in events {
        write!(f, "{}", event)?;
    }

    Ok(())
}

fn format_attrs<'a>(f: &mut fmt::Formatter, attrs: &Vec<Attribute<'a>>) -> fmt::Result {
    for attr in attrs {
        write!(f, " {}=\"{}\"", attr.0, Escape(&attr.1))?;
    }

    Ok(())
}

impl<'a> Builder<'a> {
    pub(crate) fn new() -> Self {
        Builder { events: Vec::new() }
    }

    pub(crate) fn consume(self) -> Output<'a> {
        Output { events: self.events }
    }

    pub fn tag_start<'b, N>(&'b mut self, name: N) -> TagStartBuilder<'b, 'a>
    where
        N: Into<Cow<'a, str>>,
    {
        TagStartBuilder {
            name: name.into(),
            attrs: Vec::new(),
            builder: self,
        }
    }

    pub fn text<T>(&mut self, text: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.events.push(Event::Text { text: text.into() });

        self
    }

    pub fn tag_end<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.events.push(Event::TagEnd { name: name.into() });

        self
    }

    fn append(&mut self, event: Event<'a>) {
        self.events.push(event);
    }
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Event::Text { ref text } => write!(f, "{}", Escape(text)),
            &Event::TagStart {
                ref name,
                ref attrs,
            } => {
                write!(f, "<{}", name)
                    .and_then(|_| format_attrs(f, attrs))
                    .and_then(|_| write!(f, ">"))
            }
            &Event::TagEnd { ref name } => write!(f, "</{}>", name),
        }
    }
}

impl<'a> fmt::Display for Output<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_events(f, &self.events)
    }
}

impl<'a, 'b> TagStartBuilder<'a, 'b> {
    pub fn add_attr<N, V>(&mut self, name: N, value: V) -> &mut Self
    where
        N: Into<Cow<'b, str>>,
        V: Into<Cow<'b, str>>,
    {
        self.attrs.push((name.into(), value.into()));

        self
    }

    pub fn finish(&'a mut self) -> &'a mut Builder<'b> {
        let mut attrs = vec![];

        mem::swap(&mut self.attrs, &mut attrs);

        self.builder.append(Event::TagStart {
            attrs,
            name: self.name.clone(),
        });

        self.builder
    }
}