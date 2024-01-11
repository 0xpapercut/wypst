use crate::node::Node;
use crate::katex::{self, LapBuilder, MClassBuilder};

pub fn not() -> Node {
    Node::Node(katex::AtomBuilder::default()
        .family(katex::AtomGroup::Rel)
        .text("\\@not".to_string())
        .build().unwrap().into_node())
}

pub fn equals() -> Node {
    Node::Node(katex::AtomBuilder::default()
        .family(katex::AtomGroup::Rel)
        .text("=".to_string())
        .build().unwrap().into_node())
}

pub fn neq() -> Node {
    let not = MClassBuilder::default()
        .mclass("rel".to_string())
        .body([
            LapBuilder::default()
                .alignment("rlap".to_string())
                .body(Box::new(not().into_ordgroup(katex::Mode::Math).into_node()))
                .build().unwrap().into_node()
        ].to_vec())
        .is_character_box(false)
        .build().unwrap().into_node();
    Node::Node(MClassBuilder::default()
        .mclass("mrel".to_string())
        .is_character_box(false)
        .body([not, equals().into_node().unwrap()].to_vec())
        .build().unwrap().into_node())
}

pub fn define() -> Node {
    Node::Array([
        katex::Symbol::get(katex::Mode::Math, ':').create_node(),
        katex::Symbol::get(katex::Mode::Math, '=').create_node(),
    ].to_vec())
}
