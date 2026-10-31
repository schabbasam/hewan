
pub enum Node {
    File {
        name: String,
        content: String
    },
    Directory {
        name: String,
        children: Vec<Node>
    },
    Message {
        name: String
    }
}

impl Node {
    pub fn get_name(&self) -> &str {
        // if a multiple patterns should do the same thing, you can use '|'
        match self {
            Node::File { name }
            | Node::Directory { name }
            | Node::Message { name } => name
        }
    }

    pub fn is_directory(&self) -> bool {
        matches!(self, Node::Directory {..})
    }
}
