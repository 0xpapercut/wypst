use typst;

use crate::katex;
use crate::node::*;
use crate::content::*;

pub fn convert(root: &typst::foundations::Content) -> Node {
    let styles = typst::foundations::StyleChain::default();
    let mut converter = ContentConverter {
        styles: styles,
    };
    root.accept(&mut converter)
}

#[derive(Clone)]
pub struct ContentConverter<'a> {
    pub styles: typst::foundations::StyleChain<'a>,
}

impl ContentVisitor for ContentConverter<'_> {
    fn visit_equation(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to::<typst::math::EquationElem>().unwrap();
        Node::Array(elem.body().accept(self).as_array())
    }

    fn visit_op(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_mat(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_vec(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_frac(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_align_point(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_linebreak(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_sequence(&mut self, content: &typst::foundations::Content) -> Node {
        let mut sequence_converter = SequenceConverter::new(content);
        sequence_converter.convert(self)
    }

    fn visit_space(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_text(&mut self, content: &typst::foundations::Content) -> Node {
        let mut text_converter = TextConverter::new(content.to_text());
        text_converter.convert()
    }

    fn visit_lr(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_attach(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_math_style(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_h(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }
}

pub struct SequenceConverter<'a> {
    pub content: &'a typst::foundations::Content,
}

impl<'a> SequenceConverter<'a> {
    pub fn new(content: &'a typst::foundations::Content) -> Self {
        Self {
            content,
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter) -> Node {
        let sequence = self.content.to_sequence().unwrap();
        Node::Array(sequence.map(|e| e.accept(visitor).as_array()).flatten().collect())
    }
}

pub struct TextConverter<'a> {
    pub elem: &'a typst::text::TextElem,
}

impl<'a> TextConverter<'a> {
    pub fn new(elem: &'a typst::text::TextElem) -> Self {
        Self {
            elem,
        }
    }

    pub fn convert(&mut self) -> Node {
        let text = self.elem.text();
        if text.chars().count() == 1 {
            let name = text.chars().next().unwrap();
            self.convert_char(name, katex::Mode::Math)
        } else {
            self.convert_text(text)
        }
    }

    pub fn convert_text(&mut self, text: &str) -> Node {
        let body = text.chars().map(|name| katex::Symbol::get(katex::Mode::Text, name).create_node()).collect();
        Node::Node(katex::Node::Text(katex::TextBuilder::default()
            .body(body)
            .build().unwrap()
        ))
    }

    pub fn convert_char(&mut self, name: char, mode: katex::Mode) -> Node {
        Node::Node(katex::Symbol::get(mode, name).create_node())
    }
}
