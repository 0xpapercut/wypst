use serde::Serialize;
use crate::katex::{self, Align};

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum Node {
    Node(katex::Node),
    Array(katex::NodeArray),
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Node(node) => write!(f, "{:?}", node),
            Node::Array(array) => write!(f, "{:?}", array),
        }
    }
}

impl Node {
    pub fn into_node(self) -> Result<katex::Node, &'static str> {
        match self {
            Node::Node(node) => Ok(node),
            Node::Array(array) => {
                if array.len() == 1 {
                    Ok(array.iter().next().cloned().unwrap())
                } else {
                    Err("Cannot convert an array with more than one element to a single node")
                }
            },
        }
    }

    pub fn into_ordgroup(&self, mode: katex::Mode) -> katex::OrdGroup {
        katex::OrdGroupBuilder::default()
            .mode(mode)
            .body(self.clone().into_array())
            .build().unwrap()
    }

    pub fn into_node_fallback_ordgroup(&self, mode: katex::Mode) -> katex::Node {
        match self.clone().into_node() {
            Ok(node) => node,
            Err(_) => self.into_ordgroup(mode).into_node(),
        }
    }

    pub fn into_array(self) -> katex::NodeArray {
        match self {
            Node::Node(node) => vec![node.clone()],
            Node::Array(array) => array,
        }
    }

    pub fn join(&mut self, node: Node) {
        let mut arr = self.clone().as_array();
        arr.append(&mut node.as_array());
        *self = Node::Array(arr);
    }
}
