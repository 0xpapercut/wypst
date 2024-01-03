use typst;

use crate::katex;
use crate::node::*;
use crate::content::*;
use crate::ext::*;

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
        let elem = content.to_equation();
        Node::Array(elem.body().accept(self).as_array())
    }

    fn visit_op(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_mat(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_mat();
        let mut constructor = katex::ArrayConstructor::default();

        for row in elem.rows() {
            constructor.next_row();
            for content in row {
                let node = content.accept(self);
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.as_array())
                    .build().unwrap().into_node();
                let styling = katex::StylingBuilder::default()
                    .body([ordgroup].to_vec())
                    .style(katex::StyleStr::Text)
                    .build().unwrap().into_node();
                constructor.push_node(styling);
            }
        }
        let array = constructor.builder().build().unwrap().into_node();
        let delim = elem.delim(self.styles).unwrap();
        let leftright = katex::LeftRightBuilder::default()
            .body([array].to_vec())
            .left(delim.open().to_string())
            .right(delim.close().to_string())
            .build().unwrap().into_node();
        Node::Node(leftright)
    }

    fn visit_vec(&mut self, content: &typst::foundations::Content) -> Node {
        let mut converter = VecConverter::new(content.to_vec());
        converter.convert(self)
    }

    fn visit_frac(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_frac();

        let numer_body = elem.num().accept(self).as_array();
        let numer = katex::OrdGroupBuilder::default()
            .body(numer_body)
            .build().unwrap().into_node();

        let denom_body = elem.denom().accept(self).as_array();
        let denom = katex::OrdGroupBuilder::default()
            .body(denom_body)
            .build().unwrap().into_node();

        let genfrac = katex::GenFracBuilder::default()
            .numer(Box::new(numer))
            .denom(Box::new(denom))
            .build().unwrap().into_node();

        Node::Node(genfrac)
    }

    fn visit_align_point(&mut self, content: &typst::foundations::Content) -> Node {
        let ordgroup = katex::OrdGroupBuilder::default().build().unwrap().into_node();
        Node::Node(ordgroup)
    }

    fn visit_linebreak(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_sequence(&mut self, content: &typst::foundations::Content) -> Node {
        let mut converter = SequenceConverter::new(content);
        converter.convert(self)
    }

    fn visit_space(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_text(&mut self, content: &typst::foundations::Content) -> Node {
        let mut text_converter = TextConverter::new(content.to_text());
        text_converter.convert()
    }

    fn visit_lr(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_lr();

        let mut body = self.visit_sequence(elem.body()).as_array();
        let left = match body.remove(0) {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };
        let right = match body.pop().unwrap() {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };
        let leftright = katex::LeftRightBuilder::default()
            .body(body)
            .left(left)
            .right(right)
            .build().unwrap().into_node();
        Node::Node(leftright)
    }

    fn visit_attach(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_attach();

        let base = elem.base().accept(self).as_node().map(Box::new).ok();
        let sup = elem.t(self.styles).map(|n| n.accept(self).as_node().map(Box::new).ok()).flatten();
        let sub = elem.b(self.styles).map(|n| n.accept(self).as_node().map(Box::new).ok()).flatten();

        let subsup = katex::SupSubBuilder::default()
            .base(base)
            .sup(sup)
            .sub(sub)
            .build().unwrap().into_node();
        Node::Node(subsup)
    }

    fn visit_math_style(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_math_style();

        let body = elem.body().accept(self).as_node().map(Box::new).unwrap();
        let font = match elem.variant(self.styles) {
            Some(typst::math::MathVariant::Bb) => "mathbb",
            Some(typst::math::MathVariant::Cal) => "mathcal",
            Some(typst::math::MathVariant::Serif) => unimplemented!(),
            Some(typst::math::MathVariant::Sans) => unimplemented!(),
            Some(typst::math::MathVariant::Frak) => unimplemented!(),
            Some(typst::math::MathVariant::Mono) => unimplemented!(),
            None => "mathrm"
        }.to_string();

        Node::Node(katex::FontBuilder::default()
            .body(body)
            .font(font)
            .build().unwrap().into_node()
        )
    }

    fn visit_h(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }
}

pub struct SequenceConverter<'a> {
    pub content: &'a typst::foundations::Content,
    pub body: Vec<Vec<Node>>,
    pub stack: Vec<Node>,
    pub is_aligned: bool,
}

impl<'a> SequenceConverter<'a> {
    pub fn new(content: &'a typst::foundations::Content) -> Self {
        Self {
            content,
            body: Vec::new(),
            stack: Vec::new(),
            is_aligned: false,
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter) -> Node {
        self.process_sequence_elements(visitor);

        if self.is_aligned {
            self.convert_align()
        } else {
            self.convert_flatten()
        }
    }

    pub fn convert_flatten(&mut self) -> Node {
        let nodes = self.body.iter().flatten().map(|n| n.clone().as_array()).flatten();
        Node::Array(nodes.collect())
    }

    pub fn convert_align(&mut self) -> Node {
        let mut constructor = katex::ArrayConstructor::default();

        for row in self.body.iter_mut() {
            constructor.next_row();
            for node in row.iter_mut() {
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.clone().as_array())
                    .build().unwrap().into_node();
                let styling = katex::StylingBuilder::default()
                    .style(katex::StyleStr::Display)
                    .body([ordgroup].to_vec())
                    .build().unwrap().into_node();
                constructor.push_node(styling)
            }
        }
        constructor.cols_leftright_align();

        let array = constructor.builder()
            .add_jot(true)
            .leqno(false)
            .col_separation_type(katex::ColSeparationType::Align)
            .build().unwrap().into_node();
        Node::Node(array)
    }

    pub fn process_sequence_elements(&mut self, visitor: &mut ContentConverter) {
        let sequence = self.content.to_sequence().unwrap();

        for elem in sequence {
            if elem.is_linebreak() || elem.is_align_point() {
                self.dump_stack_onto_body();
                if elem.is_linebreak() {
                    self.body.push(Vec::new());
                }
                if elem.is_align_point() {
                    self.is_aligned = true;
                }
            }
            let node = elem.accept(visitor);
            self.stack.push(node);
        }
        self.dump_stack_onto_body();
    }

    pub fn dump_stack_onto_body(&mut self) {
        if self.body.is_empty() {
            self.body.push(Vec::new())
        }
        let nodes = self.stack.iter().map(|n| n.clone().as_array()).flatten().collect();
        self.body.last_mut().unwrap().push(Node::Array(nodes));
        self.stack.clear();
    }
}

pub struct VecConverter<'a> {
    pub elem: &'a typst::math::VecElem,
}

impl<'a> VecConverter<'a> {
    pub fn new(elem: &'a typst::math::VecElem) -> Self {
        Self {
            elem,
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter) -> Node {
        let mut constructor = katex::ArrayConstructor::default();

        for content in self.elem.children() {
            constructor.next_row();
            let node = content.accept(visitor).as_node().unwrap();
            let ordgroup = katex::OrdGroupBuilder::default()
                .body([node].to_vec())
                .build().unwrap().into_node();
            let styling = katex::StylingBuilder::default()
                .body([ordgroup].to_vec())
                .style(katex::StyleStr::Text)
                .build().unwrap().into_node();
            constructor.push_node(styling);
        }
        constructor.cols_center_align();

        let mut builder = constructor.builder();
        let array = builder
            .hskip_before_and_after(false)
            .row_gaps([None].to_vec())
            .build().unwrap().into_node();
        let delim = self.elem.delim(visitor.styles).unwrap();
        let leftright = katex::LeftRightBuilder::default()
            .body([array].to_vec())
            .left(delim.open().to_string())
            .right(delim.close().to_string())
            .build().unwrap().into_node();
        Node::Node(leftright)
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

        let text = katex::TextBuilder::default()
            .body(body)
            .build().unwrap().into_node();
        Node::Node(text)
    }

    pub fn convert_char(&mut self, name: char, mode: katex::Mode) -> Node {
        Node::Node(katex::Symbol::get(mode, name).create_node())
    }
}
