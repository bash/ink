use std::fmt;

pub struct Builder<'a> {
    nodes: Vec<Node<'a>>,
}

pub struct Output<'a> {
    nodes: Vec<Node<'a>>,
}

// We're not exposing `Node` directly which allows us
// to make changes to the representation without breaking api compatiblity
// Consumers have to use the respective methods on a `Builder` instance instead.
enum Node<'a> {
    /// TODO: A tag needs to allow children
    /// TODO: A tag needs attributes
    Tag {
        name: &'a str,
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

impl<'a> Builder<'a> {
    pub(crate) fn new() -> Self {
        Builder { nodes: Vec::new() }
    }

    pub(crate) fn consume(self) -> Output<'a> {
        Output { nodes: self.nodes }
    }

    pub fn tag_with_text(&mut self, name: &'a str, text: String) {
        self.nodes.push(Node::Tag {
            name,
            children: vec![Node::Text { text }],
        })
    }
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // TODO: escaping
            &Node::Text { ref text } => write!(f, "{}", text),
            &Node::Tag { name, ref children } => {
                write!(f, "<{}>", name)
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