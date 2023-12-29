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

pub trait ContentVisitor {
    fn visit_equation(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_sequence(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_text(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_space(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_lr(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_attach(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_math_style(&mut self, content: &typst::foundations::Content) -> Node;
    fn visit_h(&mut self, content: &typst::foundations::Content) -> Node;
}

impl ContentVisitor for TypstToKatexConverter<'_> {
    fn visit_equation(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to::<typst::math::EquationElem>().unwrap();
        Node::Array(elem.body().accept(self).as_array())
    }

    fn visit_sequence(&mut self, content: &typst::foundations::Content) -> Node {
        // let mut is_aligned = false;
        let mut nodes: katex::NodeArray = Vec::new();
        for elem in content.to_sequence().unwrap() {
            let node = elem.accept(self);
            match node {
                Node::Node(n) => nodes.push(n),
                Node::Array(arr) => nodes.extend(arr),
            }
        }
        return Node::Array(nodes);
        // for elem in content.to_sequence().unwrap() {
        //     let node = match elem {
        //         _ if elem.is::<typst::math::AlignPointElem>() => { // \begin{aligned}x&=y\\&=z\end{aligned}
        //             is_aligned = true;
        //             let non_style_nodes: katex::NodeArray;
        //             (nodes, non_style_nodes) = nodes.into_iter().partition(|n| match n {
        //                 katex::Node::Styling(_) => true,
        //                 _ => false,
        //             });
        //             Node::Node(katex::Node::Styling(katex::Styling {
        //                 mode: self.mode,
        //                 loc: None,
        //                 style: katex::StyleStr::Display,
        //                 body: non_style_nodes,
        //             }))
        //         },
        //         _ => elem.accept(self),
        //     };
        //     match node {
        //         Node::Node(n) => nodes.push(n),
        //         Node::Array(arr) => nodes.extend(arr),
        //     }
        // }
        // if is_aligned {
        //     nodes = [
        //         katex::Node::Array(katex::Array {
        //             mode: self.mode,
        //             loc: None,
        //             col_separation_type: Some(katex::ColSeparationType::AlignAt),
        //             hskip_before_and_after: None,
        //             add_jot: None,
        //             cols: None,
        //             arraystretch: 1.0,
        //             body: [nodes].to_vec(),
        //             row_gaps: Vec::new(),
        //             h_lines_before_row: [[].to_vec()].to_vec(),
        //             tags: None,
        //             leqno: None,
        //             is_cd: None,
        //         }),
        //     ].to_vec();
        // }
        // Node::Array(nodes)
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
            Some(t) => t.accept(self).as_node().map(Box::new).ok(),
            None => None
        };
        // let sub = match elem.b(style_chain) {
        //     Some(t) => t.accept(self).as_node().map(Box::new).ok(),
        //     None => None
        // };
        let sub = katex::Node::OrdGroup(katex::OrdGroup {
            mode: self.mode,
            loc: None,
            body: elem.b(style_chain).unwrap().accept(self).as_array(),
            semisimple: None,
        });

        Node::Node(katex::Node::SupSub(katex::SupSub {
            mode: self.mode,
            loc: None,
            base,
            sup,
            sub: Some(Box::new(sub)),
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

// pub trait ExprVisitor {
//     fn visit_text(&mut self, expr: &typst::syntax::ast::Text) -> Node;
//     fn visit_math(&mut self, expr: &typst::syntax::ast::Math) -> Node;
//     fn visit_math_delimited(&mut self, expr: &typst::syntax::ast::MathDelimited) -> Node;
//     fn visit_math_frac(&mut self, expr: &typst::syntax::ast::MathFrac) -> Node;
//     fn visit_math_ident(&mut self, expr: &typst::syntax::ast::MathIdent) -> Node;
//     fn visit_field_access(&mut self, expr: &typst::syntax::ast::FieldAccess) -> Node;
//     fn visit_math_attach(&mut self, expr: &typst::syntax::ast::MathAttach) -> Node;
// }

// impl ExprVisitor for TypstToKatexConverter<'_> {
//     fn visit_text(&mut self, expr: &typst::syntax::ast::Text) -> Node {
//         // Suppose the text expression is made of a single character
//         let c = expr.get().chars().next().unwrap();
//         Node::Node(katex::Symbol::get(katex::Mode::Math, c).create_node())
//     }

//     fn visit_math(&mut self, expr: &typst::syntax::ast::Math) -> Node {
//         let exprs: Vec<_> = expr.exprs().filter(|x| !self.math_expr_filter.contains(&x.to_untyped().kind())).collect();
//         let mut nodes = Vec::new();
//         for expr_ in exprs {
//             let node = expr_.accept(self);
//             match node {
//                 Node::Node(node_) => nodes.push(node_),
//                 Node::Array(array) => nodes.extend(array),
//             }
//         }
//         Node::Array(nodes)
//     }

//     fn visit_math_delimited(&mut self, expr: &typst::syntax::ast::MathDelimited) -> Node {
//         let left = expr.open().to_untyped().text().to_string();
//         let right = expr.close().to_untyped().text().to_string();
//         Node::Node(katex::Node::LeftRight(katex::LeftRight {
//             mode: katex::Mode::Math,
//             loc: None,
//             body: typst::syntax::ast::Expr::Math(expr.body()).accept(self).as_array(),
//             left: left,
//             right: right,
//             right_color: None,
//         }))
//     }

//     fn visit_math_frac(&mut self, expr: &typst::syntax::ast::MathFrac) -> Node {
//         let get_body = |expr: typst::syntax::ast::Expr| -> katex::NodeArray {
//             let body = match expr {
//                 typst::syntax::ast::Expr::Math(math_expr) => math_expr.exprs().next().unwrap(),
//                 _ => expr
//             };
//             body.accept(&mut self.clone()).as_array()
//         };
//         let numer = katex::OrdGroup {
//             mode: self.mode,
//             loc: None,
//             body: get_body(expr.num()),
//             semisimple: None,
//         };
//         let denom = katex::OrdGroup {
//             mode: self.mode,
//             loc: None,
//             body: get_body(expr.denom()),
//             semisimple: None,
//         };
//         Node::Node(katex::Node::GenFrac(katex::GenFrac {
//             mode: self.mode,
//             loc: None,
//             continued: false,
//             numer: Box::new(katex::Node::OrdGroup(numer)),
//             denom: Box::new(katex::Node::OrdGroup(denom)),
//             has_bar_line: true,
//             left_delim: None,
//             right_delim: None,
//             size: katex::GenFracSizeType::Auto,
//             bar_size: None,
//         }))
//     }

//     fn visit_math_ident(&mut self, expr: &typst::syntax::ast::MathIdent) -> Node {
//         // TODO: treat operators like `lim`
//         let value = self.scope.get(expr.as_str()).unwrap();
//         match value {
//             typst::foundations::Value::Symbol(symbol) => Node::Node(katex::Symbol::get(self.mode, symbol.get()).create_node()),
//             // typst::eval::Value::Content(content) => panic!("Debug: {:#?}", content),
//             _ => panic!("Value of type `{:?}` not supported.", value.ty())
//         }
//     }

//     fn visit_field_access(&mut self, expr: &typst::syntax::ast::FieldAccess) -> Node {
//         let mut idents = Vec::new();
//         let value;
//         let mut target = *expr;
//         loop {
//             idents.push(target.field());
//             match target.target() {
//                 typst_syntax::ast::Expr::FieldAccess(ref field_access) => { target = field_access.clone() },
//                 typst_syntax::ast::Expr::MathIdent(math_ident) => { value = self.scope.get(math_ident.as_str()); break; },
//                 _ => panic!("Not implemented."),
//             }
//         }
//         let mut symbol = match value.unwrap() {
//             typst::foundations::Value::Symbol(symbol) => symbol.clone(),
//             _ => panic!("Not implemented."),
//         };
//         for ident in idents.iter().rev() {
//             symbol = symbol.modified(ident.as_str()).unwrap();
//         }
//         Node::Node(katex::Symbol::get(self.mode, symbol.get()).create_node())
//     }

//     fn visit_math_attach(&mut self, expr: &typst::syntax::ast::MathAttach) -> Node {
//         let base = Some(Box::new(expr.base().accept(self).as_node().unwrap()));
//         let sup = expr.top().map(|top| Box::new(top.accept(self).as_node().unwrap()));
//         let sub = expr.bottom().map(|top| Box::new(top.accept(self).as_node().unwrap()));
//         Node::Node(katex::Node::SupSub(katex::SupSub { mode: self.mode, loc: None, base, sup, sub }))
//     }
// }

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
            _ if self.is_sequence() => visitor.visit_sequence(self),
            _ => panic!("Content element `{:#?}` not implemented yet.", self),
        }
    }
}

// impl<'a> ExprExt for typst::syntax::ast::Expr<'a> {
//     fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> Node {
//         match self {
//             typst::syntax::ast::Expr::Math(expr) => visitor.visit_math(expr),
//             typst::syntax::ast::Expr::Text(expr) => visitor.visit_text(expr),
//             typst::syntax::ast::Expr::MathFrac(expr) => visitor.visit_math_frac(expr),
//             typst::syntax::ast::Expr::MathDelimited(expr) => visitor.visit_math_delimited(expr),
//             typst::syntax::ast::Expr::MathIdent(expr) => visitor.visit_math_ident(expr),
//             typst::syntax::ast::Expr::FieldAccess(expr) => visitor.visit_field_access(expr),
//             typst::syntax::ast::Expr::MathAttach(expr) => visitor.visit_math_attach(expr),
//             _ => panic!("Support for {:?} not implemented.", self.to_untyped().kind()),
//         }
//     }
// }
