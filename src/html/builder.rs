use std::fmt;
use std::mem;
use super::escape::Escape;

#[derive(Debug)]
pub struct Builder<'a> {
    nodes: Vec<Node<'a>>,
}

#[derive(Debug)]
pub struct TagBuilder<'a, 'b: 'a> {
    name: &'b str,
    attrs: Vec<Attribute<'b>>,
    children: Vec<Node<'b>>,
    builder: &'a mut Builder<'b>,
}

#[derive(Debug)]
pub struct Output<'a> {
    nodes: Vec<Node<'a>>,
}

type Attribute<'a> = (&'a str, &'a str);

// We're not exposing `Node` directly which allows us
// to make changes to the representation without breaking api compatiblity
// Consumers have to use the respective methods on a `Builder` instance instead.
#[derive(Debug)]
enum Node<'a> {
    /// TODO: A tag needs to allow children
    /// TODO: A tag needs attributes
    Tag {
        name: &'a str,
        attrs: Vec<Attribute<'a>>,
        children: Vec<Node<'a>>,
    },
    /// TODO: Change text type to &'a str
    Text { text: String },
}

fn format_nodes<'a>(f: &mut fmt::Formatter, nodes: &Vec<Node<'a>>) -> fmt::Result {
    for node in nodes {
        write!(f, "{}", node)?;
    }

    Ok(())
}

fn format_attrs<'a>(f: &mut fmt::Formatter, attrs: &Vec<Attribute<'a>>) -> fmt::Result {
    for attr in attrs {
        write!(f, " {}=\"{}\"", attr.0, Escape(attr.1))?;
    }

    Ok(())
}

impl<'a> Builder<'a> {
    pub(crate) fn new() -> Self {
        Builder { nodes: Vec::new() }
    }

    pub(crate) fn consume(self) -> Output<'a> {
        Output { nodes: self.nodes }
    }

    pub fn tag<'b>(&'b mut self, name: &'a str) -> TagBuilder<'b, 'a> {
        TagBuilder {
            name,
            attrs: Vec::new(),
            children: Vec::new(),
            builder: self,
        }
    }
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // TODO: escaping
            &Node::Text { ref text } => write!(f, "{}", Escape(text)),
            &Node::Tag {
                name,
                ref children,
                ref attrs,
            } => {
                write!(f, "<{}", name)
                    .and_then(|_| format_attrs(f, attrs))
                    .and_then(|_| write!(f, ">"))
                    .and_then(|_| format_nodes(f, children))
                    .and_then(|_| write!(f, "</{0}>", name))
            }
        }
    }
}

impl<'a> fmt::Display for Output<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_nodes(f, &self.nodes)
    }
}

impl<'a, 'b> TagBuilder<'a, 'b> {
    pub fn add_attr(&mut self, name: &'b str, value: &'b str) -> &mut TagBuilder<'a, 'b> {
        self.attrs.push((name, value));

        self
    }

    pub fn add_text(&mut self, text: String) -> &mut TagBuilder<'a, 'b> {
        self.children.push(Node::Text { text });

        self
    }

    pub fn finish(&mut self) {
        let mut attrs = vec![];
        let mut children = vec![];

        mem::swap(&mut self.attrs, &mut attrs);
        mem::swap(&mut self.children, &mut children);

        self.builder.nodes.push(Node::Tag {
            attrs,
            name: self.name,
            children,
        })
    }
}