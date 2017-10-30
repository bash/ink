use std::fmt;
use std::mem;
use std::borrow::Cow;
use super::output::Output;
use super::escape::Escape;

#[derive(Debug)]
pub struct Builder {
    events: Vec<Event>,
}

#[derive(Debug)]
pub struct TagStartBuilder<'a> {
    name: Cow<'static, str>,
    attrs: Vec<Attribute<'static>>,
    builder: &'a mut Builder,
}

type Attribute<'a> = (Cow<'a, str>, Cow<'a, str>);

// We're not exposing `Event` directly which allows us
// to make changes to the representation without breaking api compatiblity
// Consumers have to use the respective methods on a `Builder` instance instead.
#[derive(Debug)]
pub(crate) enum Event {
    TagStart {
        name: Cow<'static, str>,
        attrs: Vec<Attribute<'static>>,
    },
    Text { text: Cow<'static, str> },
    TagEnd { name: Cow<'static, str> },
}

fn format_attrs<'a>(f: &mut fmt::Formatter, attrs: &Vec<Attribute<'a>>) -> fmt::Result {
    for attr in attrs {
        write!(f, " {}=\"{}\"", attr.0, Escape(&attr.1))?;
    }

    Ok(())
}

impl Builder {
    pub(crate) fn new() -> Self {
        Builder { events: Vec::new() }
    }

    pub(crate) fn consume(self) -> Output {
        Output::new(self.events)
    }

    pub fn tag_start<'a, N>(&'a mut self, name: N) -> TagStartBuilder<'a>
    where
        N: Into<Cow<'static, str>>,
    {
        TagStartBuilder {
            name: name.into(),
            attrs: Vec::new(),
            builder: self,
        }
    }

    pub fn text<T>(&mut self, text: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.events.push(Event::Text { text: text.into() });

        self
    }

    pub fn tag_end<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.events.push(Event::TagEnd { name: name.into() });

        self
    }

    fn append(&mut self, event: Event) {
        self.events.push(event);
    }
}

impl<'a> fmt::Display for Event {
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

impl<'a> TagStartBuilder<'a> {
    pub fn add_attr<N, V>(&mut self, name: N, value: V) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.attrs.push((name.into(), value.into()));

        self
    }

    pub fn finish(&'a mut self) -> &'a mut Builder {
        let mut attrs = vec![];

        mem::swap(&mut self.attrs, &mut attrs);

        self.builder.append(Event::TagStart {
            attrs,
            name: self.name.clone(),
        });

        self.builder
    }
}
