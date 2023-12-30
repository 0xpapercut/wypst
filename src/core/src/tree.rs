use serde::Serialize;

use typst;
use typst::foundations::NativeElement;
use typst_syntax::ast::AstNode;
use typst::Library;

use crate::katex;
use crate::log;
use crate::utils::*;
use crate::node::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn convert(root: &typst::foundations::Content) -> Node {
    let styles = typst::foundations::StyleChain::default();
    let mut converter = TypstToKatexConverter {
        mode: katex::Mode::Math,
        styles: styles,
    };
    root.accept(&mut converter)
}

#[derive(Clone)]
pub struct TypstToKatexConverter<'a> {
    pub mode: katex::Mode,
    pub styles: typst::foundations::StyleChain<'a>,
}

pub struct SequenceToKatexConverter<'a> {
    pub sequence: &'a typst::foundations::Content,
    pub converter: TypstToKatexConverter<'a>,
    pub is_aligned: bool,
    pub is_array: bool,
    pub rows: Vec<Vec<Node>>,
}

impl<'a> SequenceToKatexConverter<'a> {
    pub fn new(sequence: &'a typst::foundations::Content, converter: TypstToKatexConverter<'a>) -> Self {
        Self {
            sequence,
            converter,
            is_aligned: false,
            is_array: false,
            rows: Vec::new(),
        }
    }

    pub fn convert(&mut self) -> Node {
        self.add_row();
        for elem in self.sequence.to_sequence().unwrap() {
            if elem.is::<typst::math::AlignPointElem>() {
                self.set_align_point();
            }
            if elem.is::<typst::text::LinebreakElem>() {
                self.set_linebreak();
                continue;
            }
            let node = elem.accept(&mut self.converter);
            println!("\n\n{:#?}", elem);
            println!("- produces -");
            println!("{:#?}\n\n", node);
            self.push(node);
        }
        if self.rows.iter().all(|row| row.len() <= 1) {
            let mut array: katex::NodeArray = Vec::new();
            for (i, row) in self.rows.iter().enumerate() {
                for elem in row {
                    array.extend(elem.clone().as_array());
                }
                if i < self.rows.len() - 1 {
                    let cr = katex::Cr::default();
                    array.push(katex::Node::Cr(cr));
                }
            }
            return Node::Array(array);
        }
        // Treat object as an array.
        let mut body: katex::NodeArray2D = Vec::new();
        for row in &self.rows {
            body.push(Vec::new());
            for elem in row {
                let mut styling = katex::Styling::default();
                if self.is_aligned {
                    let mut ordgroup = katex::OrdGroup::default();
                    ordgroup.body = elem.clone().as_array();
                    styling.body = [katex::Node::OrdGroup(ordgroup)].to_vec();
                } else {
                    styling.body = elem.clone().as_array();
                }
                body.last_mut().unwrap().push(katex::Node::Styling(styling));
            }
        }
        let mut array = katex::Array::default();
        array.body = body;
        if self.is_aligned {
            let mut cols: Vec<katex::AlignSpec> = Vec::new();
            for i in 0..self.count_columns() {
                let align = katex::Align {
                    align: if i % 2 == 0 { "r".to_string() } else { "l".to_string() },
                    pregap: if i > 1 && i % 2 == 0 { Some(1f32) } else { Some(0f32) },
                    postgap: Some(0f32),
                };
                cols.push(katex::AlignSpec::Align(align));
            }
            array.cols = Some(cols);
            array.h_lines_before_row = vec![Vec::new(); self.rows.len() + 1];
            array.add_jot = Some(true);
            array.col_separation_type = Some(katex::ColSeparationType::Align);
            array.leqno = Some(false);
        }
        Node::Node(katex::Node::Array(array))
    }

    pub fn push(&mut self, node: Node) {
        if self.is_current_row_empty() {
            self.add_row_element();
        }
        let row = self.get_current_row();

        row.last_mut().unwrap().join(node);
    }

    pub fn add_row(&mut self) {
        self.rows.push(Vec::new());
    }

    pub fn add_row_element(&mut self) {
        self.rows.last_mut().unwrap().push(Node::Array(Vec::new()));
    }

    pub fn is_current_row_empty(&self) -> bool {
        self.rows.last().unwrap().is_empty()
    }

    pub fn get_current_row(&mut self) -> &mut Vec<Node> {
        self.rows.last_mut().unwrap()
    }

    pub fn set_linebreak(&mut self) {
        self.add_row();
    }

    pub fn set_align_point(&mut self) {
        self.is_aligned = true;
        self.add_row_element();
    }

    pub fn count_columns(&self) -> usize {
        self.rows.iter().map(|row| row.len()).max().unwrap_or(0)
    }
}

pub trait ContentVisitor {
    fn visit_equation(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_sequence(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_text(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_space(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_lr(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_attach(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_math_style(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_h(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_linebreak(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_align_point(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_frac(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_vec(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_mat(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_op(&mut self, content: &typst::foundations::Content) -> Node;

}

impl ContentVisitor for TypstToKatexConverter<'_> {
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
        let elem = content.to::<typst::math::VecElem>().unwrap();
        let delim = elem.delim(self.styles).unwrap();
        unimplemented!()
    }

    fn visit_frac(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to::<typst::math::FracElem>().unwrap();
        let numer = katex::Node::OrdGroup(katex::OrdGroup {
            mode: katex::Mode::Math,
            loc: None,
            body: elem.num().accept(self).as_array(),
            semisimple: None,
        });
        let denom = katex::Node::OrdGroup(katex::OrdGroup {
            mode: katex::Mode::Math,
            loc: None,
            body: elem.denom().accept(self).as_array(),
            semisimple: None,
        });
        Node::Node(katex::Node::GenFrac(katex::GenFrac::default(numer, denom)))
    }

    fn visit_align_point(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Node(katex::Node::OrdGroup(katex::OrdGroup::default()))
    }

    fn visit_linebreak(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_sequence(&mut self, content: &typst::foundations::Content) -> Node {
        let mut sequence_converter = SequenceToKatexConverter::new(content, self.clone());
        sequence_converter.convert()
    }

    fn visit_space(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_text(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to::<typst::text::TextElem>().unwrap();
        let text = elem.text();
        if text.chars().count() == 1 {
            let name = text.chars().next().unwrap();
            Node::Node(katex::Symbol::get(self.mode, name).create_node())
        } else {
            let body = text.chars().map(|name| katex::Symbol::get(katex::Mode::Text, name).create_node()).collect();
            Node::Node(katex::Node::Text(katex::Text {
                mode: self.mode,
                body,
                loc: None,
                font: None,
            }))
        }
    }

    fn visit_lr(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to::<typst::math::LrElem>().unwrap();
        let mut body = self.visit_sequence(elem.body()).as_array();
        let left = match body.remove(0) {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };
        let right = match body.pop().unwrap() {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };

        Node::Node(katex::Node::LeftRight(katex::LeftRight {
            mode: self.mode,
            loc: None,
            body,
            left,
            right,
            right_color: None,
        }))
    }

    fn visit_attach(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to::<typst::math::AttachElem>().unwrap();

        let styles = typst::foundations::Styles::default();
        let style_chain = typst::foundations::StyleChain::new(&styles);
        let base = elem.base().accept(self).as_node().map(Box::new).ok();
        let sup = match elem.t(style_chain) {
            Some(t) => {
                let body = t.accept(self).as_array();
                let node = katex::Node::OrdGroup(katex::OrdGroup {
                    mode: self.mode,
                    loc: None,
                    body: body,
                    semisimple: None,
                });
                Some(Box::new(node))
            }
            None => None,
        };
        let sub = match elem.b(style_chain) {
            Some(b) => {
                let body = b.accept(self).as_array();
                let node = katex::Node::OrdGroup(katex::OrdGroup {
                    mode: self.mode,
                    loc: None,
                    body: body,
                    semisimple: None,
                });
                Some(Box::new(node))
            }
            None => None,
        };
        Node::Node(katex::Node::SupSub(katex::SupSub {
            mode: self.mode,
            loc: None,
            base,
            sup,
            sub,
        }))
    }

    fn visit_math_style(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to::<typst::math::MathStyleElem>().unwrap();

        let styles = typst::foundations::Styles::default();
        let style_chain = typst::foundations::StyleChain::new(&styles);
        let body = Box::new(elem.body().accept(self).as_node().unwrap());
        let font = match elem.variant(style_chain) {
            Some(typst::math::MathVariant::Cal) => "mathcal",
            Some(typst::math::MathVariant::Bb) => "mathbb",
            None => "mathrm",
            _ => unimplemented!(),
        }.to_string();
        Node::Node(katex::Node::Font(katex::node::Font {
            mode: self.mode,
            loc: None,
            font,
            body,
        }))
    }

    fn visit_h(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }
}

pub trait ContentExt {
    fn accept<V: ContentVisitor>(&self, visitor: &mut V) -> Node;
}

impl ContentExt for typst::foundations::Content {
    fn accept<V: ContentVisitor>(&self, visitor: &mut V) -> Node {
        match self {
            _ if self.is::<typst::math::EquationElem>() => visitor.visit_equation(self),
            _ if self.is::<typst::text::SpaceElem>() => visitor.visit_space(self),
            _ if self.is::<typst::text::TextElem>() => visitor.visit_text(self),
            _ if self.is::<typst::math::LrElem>() => visitor.visit_lr(self),
            _ if self.is::<typst::math::AttachElem>() => visitor.visit_attach(self),
            _ if self.is::<typst::math::MathStyleElem>() => visitor.visit_math_style(self),
            _ if self.is::<typst::layout::HElem>() => visitor.visit_h(self),
            _ if self.is::<typst::text::LinebreakElem>() => visitor.visit_linebreak(self),
            _ if self.is::<typst::math::AlignPointElem>() => visitor.visit_align_point(self),
            _ if self.is::<typst::math::FracElem>() => visitor.visit_frac(self),
            _ if self.is::<typst::math::VecElem>() => visitor.visit_vec(self),
            _ if self.is::<typst::math::MatElem>() => visitor.visit_mat(self),
            _ if self.is::<typst::math::OpElem>() => visitor.visit_mat(self),
            _ if self.is_sequence() => visitor.visit_sequence(self),
            _ => panic!("Content element `{:#?}` not implemented yet.", self),
        }
    }
}
