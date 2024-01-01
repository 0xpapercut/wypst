use serde::Serialize;

use typst;
use typst::foundations::NativeElement;
use typst::Library;
use typst_syntax::ast::AstNode;

use crate::katex;
use crate::log;
use crate::node::*;
use crate::utils::*;

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
        let nodes = elem.children().iter().map(|e| e.accept(self));
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
        unimplemented!()
        // Node::Node(katex::Node::GenFrac(katex::GenFrac::default(numer, denom)))
    }

    fn visit_align_point(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
        // Node::Node(katex::Node::OrdGroup(katex::OrdGroup::default()))
    }

    fn visit_linebreak(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_sequence(&mut self, content: &typst::foundations::Content) -> Node {
        let mut builder = katex::ArrayBuilder::default();
        let mut stack: Vec<Node> = Vec::new();
        let mut is_aligned: bool = false;

        for elem in content.to_sequence().unwrap() {
            if elem.is_align_point() || elem.is_linebreak() {
                let mut styling = katex::StylingBuilder::default()
                    .style(katex::StyleStr::Display)
                    .body(stack.iter().map(|n| n.clone().as_node().unwrap()).collect())
                    .build()
                    .unwrap();
                let node = katex::Node::Styling(styling);
                builder.push_node(node);
                stack.clear();

                if elem.is_linebreak() {
                    builder.next_row();
                }
                if elem.is_align_point() {
                    is_aligned = true;
                }
            }
            let node = elem.accept(self);
            stack.push(node);
        }
        let mut styling = katex::StylingBuilder::default()
            .style(katex::StyleStr::Display)
            .body(stack.iter().map(|n| n.clone().as_array()).collect())
            .build()
            .unwrap();
        let node = katex::Node::Styling(styling);
        builder.push_node(node);

        Node::Node(katex::Node::Array(builder.build().unwrap()))
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
            let body = text
                .chars()
                .map(|name| katex::Symbol::get(katex::Mode::Text, name).create_node())
                .collect();
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
        }
        .to_string();
        Node::Node(katex::Node::Font(katex::node::Font {
            mode: self.mode,
            loc: None,
            font,
            body,
        }))
    }

    fn visit_h(&mut self, content: &typst::foundations::Content) -> Node {
        // TODO
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
            _ if self.is::<typst::math::OpElem>() => visitor.visit_op(self),
            _ if self.is_sequence() => visitor.visit_sequence(self),
            _ => panic!("Content element `{:#?}` not implemented yet.", self),
        }
    }
}

pub trait ContentType {
    fn is_equation(&self) -> bool;
    fn is_linebreak(&self) -> bool;
    fn is_align_point(&self) -> bool;
}

impl ContentType for typst::foundations::Content {
    fn is_equation(&self) -> bool {
        self.is::<typst::math::EquationElem>()
    }
    fn is_linebreak(&self) -> bool {
        self.is::<typst::text::LinebreakElem>()
    }
    fn is_align_point(&self) -> bool {
        self.is::<typst::math::AlignPointElem>()
    }
}
