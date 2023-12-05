use serde::Serialize;

use typst;
use typst_library;
use typst_syntax::ast::AstNode;
use typst_library::build;
use typst_library::symbols::sym;

use crate::katex;
use crate::utils::*;

pub fn convert_expr(root: typst::syntax::ast::Expr) -> Node {
    let module = build().math;
    let scope = module.scope();
    let mut converter = NodeConverter {
        scope: scope,
        mode: katex::Mode::Math,
        math_expr_filter: [
            typst::syntax::SyntaxKind::Space,
        ].to_vec(),
    };
    root.accept(&mut converter)
}

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

#[derive(Clone)]
pub struct NodeConverter<'a> {
    pub scope: &'a typst::eval::Scope,
    pub mode: katex::Mode,
    pub math_expr_filter: Vec<typst::syntax::SyntaxKind>,
}

pub trait ExprVisitor {
    fn visit_text(&mut self, expr: &typst::syntax::ast::Text) -> Node;
    fn visit_math(&mut self, expr: &typst::syntax::ast::Math) -> Node;
    fn visit_math_delimited(&mut self, expr: &typst::syntax::ast::MathDelimited) -> Node;
    fn visit_math_frac(&mut self, expr: &typst::syntax::ast::MathFrac) -> Node;
    fn visit_math_ident(&mut self, expr: &typst::syntax::ast::MathIdent) -> Node;
    fn visit_field_access(&mut self, expr: &typst::syntax::ast::FieldAccess) -> Node;
    fn visit_math_attach(&mut self, expr: &typst::syntax::ast::MathAttach) -> Node;
}

impl ExprVisitor for NodeConverter<'_> {
    fn visit_text(&mut self, expr: &typst::syntax::ast::Text) -> Node {
        // Suppose the text expression is made of a single character
        let c = expr.get().chars().next().unwrap();
        Node::Node(katex::Symbol::get(katex::Mode::Math, c).create_node())
    }

    fn visit_math(&mut self, expr: &typst::syntax::ast::Math) -> Node {
        let exprs: Vec<_> = expr.exprs().filter(|x| !self.math_expr_filter.contains(&x.to_untyped().kind())).collect();
        let mut nodes = Vec::new();
        for expr_ in exprs {
            let node = expr_.accept(self);
            match node {
                Node::Node(node_) => nodes.push(node_),
                Node::Array(array) => nodes.extend(array),
            }
        }
        Node::Array(nodes)
    }

    fn visit_math_delimited(&mut self, expr: &typst::syntax::ast::MathDelimited) -> Node {
        let left = expr.open().to_untyped().text().to_string();
        let right = expr.close().to_untyped().text().to_string();
        Node::Node(katex::Node::LeftRight(katex::LeftRight {
            mode: katex::Mode::Math,
            loc: None,
            body: typst::syntax::ast::Expr::Math(expr.body()).accept(self).as_array(),
            left: left,
            right: right,
            right_color: None,
        }))
    }

    fn visit_math_frac(&mut self, expr: &typst::syntax::ast::MathFrac) -> Node {
        let get_body = |expr: typst::syntax::ast::Expr| -> katex::NodeArray {
            let body = match expr {
                typst::syntax::ast::Expr::Math(math_expr) => math_expr.exprs().next().unwrap(),
                _ => expr
            };
            body.accept(&mut self.clone()).as_array()
        };
        let numer = katex::OrdGroup {
            mode: self.mode,
            loc: None,
            body: get_body(expr.num()),
            semisimple: None,
        };
        let denom = katex::OrdGroup {
            mode: self.mode,
            loc: None,
            body: get_body(expr.denom()),
            semisimple: None,
        };
        Node::Node(katex::Node::GenFrac(katex::GenFrac {
            mode: self.mode,
            loc: None,
            continued: false,
            numer: Box::new(katex::Node::OrdGroup(numer)),
            denom: Box::new(katex::Node::OrdGroup(denom)),
            has_bar_line: true,
            left_delim: None,
            right_delim: None,
            size: katex::GenFracSizeType::Auto,
            bar_size: None,
        }))
    }

    fn visit_math_ident(&mut self, expr: &typst::syntax::ast::MathIdent) -> Node {
        // TODO: treat operators like `lim`
        let value = self.scope.get(expr.as_str()).unwrap();
        match value {
            typst::eval::Value::Symbol(symbol) => Node::Node(katex::Symbol::get(self.mode, symbol.get()).create_node()),
            // typst::eval::Value::Content(content) => panic!("Debug: {:#?}", content),
            _ => panic!("Value of type `{:?}` not supported.", value.ty())
        }
    }

    fn visit_field_access(&mut self, expr: &typst::syntax::ast::FieldAccess) -> Node {
        let mut idents = Vec::new();
        let value;
        let mut target = *expr;
        loop {
            idents.push(target.field());
            match target.target() {
                typst_syntax::ast::Expr::FieldAccess(ref field_access) => { target = field_access.clone() },
                typst_syntax::ast::Expr::MathIdent(math_ident) => { value = self.scope.get(math_ident.as_str()); break; },
                _ => panic!("Not implemented."),
            }
        }
        let mut symbol = match value.unwrap() {
            typst::eval::Value::Symbol(symbol) => symbol.clone(),
            _ => panic!("Not implemented."),
        };
        for ident in idents.iter().rev() {
            symbol = symbol.modified(ident.as_str()).unwrap();
        }
        Node::Node(katex::Symbol::get(self.mode, symbol.get()).create_node())
    }

    fn visit_math_attach(&mut self, expr: &typst::syntax::ast::MathAttach) -> Node {
        let base = Some(Box::new(expr.base().accept(self).as_node().unwrap()));
        let sup = expr.top().map(|top| Box::new(top.accept(self).as_node().unwrap()));
        let sub = expr.bottom().map(|top| Box::new(top.accept(self).as_node().unwrap()));
        Node::Node(katex::Node::SupSub(katex::SupSub { mode: self.mode, loc: None, base, sup, sub }))
    }
}

pub trait ExprExt {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> Node;
}

impl<'a> ExprExt for typst::syntax::ast::Expr<'a> {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> Node {
        match self {
            typst::syntax::ast::Expr::Math(expr) => visitor.visit_math(expr),
            typst::syntax::ast::Expr::Text(expr) => visitor.visit_text(expr),
            typst::syntax::ast::Expr::MathFrac(expr) => visitor.visit_math_frac(expr),
            typst::syntax::ast::Expr::MathDelimited(expr) => visitor.visit_math_delimited(expr),
            typst::syntax::ast::Expr::MathIdent(expr) => visitor.visit_math_ident(expr),
            typst::syntax::ast::Expr::FieldAccess(expr) => visitor.visit_field_access(expr),
            typst::syntax::ast::Expr::MathAttach(expr) => visitor.visit_math_attach(expr),
            _ => panic!("Support for {:?} not implemented.", self.to_untyped().kind()),
        }
    }
}
