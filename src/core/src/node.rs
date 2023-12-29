use serde::Serialize;
use crate::katex::{self, Align};

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum Node {
    Node(katex::Node),
    Array(katex::NodeArray),
}

impl Node {
    pub fn as_node(self) -> Result<katex::Node, &'static str> {
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

    pub fn as_array(self) -> katex::NodeArray {
        match self {
            Node::Node(node) => vec![node.clone()],
            Node::Array(array) => array,
        }
    }
}

// pub struct AlignExpressionToArray {

// }

// impl AlignToArray {

// }
