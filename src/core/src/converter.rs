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
        let elem = content.to_equation();
        Node::Array(elem.body().accept(self).as_array())
    }

    fn visit_op(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_mat(&mut self, content: &typst::foundations::Content) -> Node {
        unimplemented!()
    }

    fn visit_vec(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_vec();

        let delim = elem.delim(self.styles).unwrap();

        // let nodes = self.visit_sequence(elem.children());

        print!("{:#?}", elem.delim(self.styles).unwrap());
        unimplemented!()
    }

    fn visit_frac(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_frac();
        let numer = katex::OrdGroupBuilder::default().body(elem.num().accept(self).as_array()).build().unwrap().into_node();
        let denom = katex::OrdGroupBuilder::default().body(elem.denom().accept(self).as_array()).build().unwrap().into_node();
        Node::Node(katex::GenFracBuilder::default()
            .numer(Box::new(numer))
            .denom(Box::new(denom))
            .build().unwrap().into_node()
        )
    }

    fn visit_align_point(&mut self, content: &typst::foundations::Content) -> Node {
        Node::Node(katex::OrdGroupBuilder::default().build().unwrap().into_node())
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

        Node::Node(katex::LeftRightBuilder::default()
            .body(body)
            .left(left)
            .right(right)
            .build().unwrap().into_node()
        )
    }

    fn visit_attach(&mut self, content: &typst::foundations::Content) -> Node {
        let elem = content.to_attach();

        let base = elem.base().accept(self).as_node().map(Box::new).ok();
        let sup = elem.t(self.styles).map(|n| n.accept(self).as_node().map(Box::new).ok()).flatten();
        let sub = elem.b(self.styles).map(|n| n.accept(self).as_node().map(Box::new).ok()).flatten();

        Node::Node(katex::SupSubBuilder::default()
            .base(base)
            .sup(sup)
            .sub(sub)
            .build().unwrap().into_node()
        )
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
    pub array: Vec<Vec<Node>>,
    pub stack: Vec<Node>,
    pub is_aligned: bool,
}

impl<'a> SequenceConverter<'a> {
    pub fn new(content: &'a typst::foundations::Content) -> Self {
        Self {
            content,
            array: Vec::new(),
            stack: Vec::new(),
            is_aligned: false,
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter) -> Node {
        self.process_sequence_elements(visitor);

        if self.count_columns() <= 1 {
            self.convert_flatten()
        } else {
            self.convert_leftright_align()
        }
    }

    pub fn process_sequence_elements(&mut self, visitor: &mut ContentConverter) {
        let sequence = self.content.to_sequence().unwrap();

        for elem in sequence {
            if elem.is_linebreak() || elem.is_align_point() {
                self.dump_stack_onto_array();

                if elem.is_linebreak() {
                    self.add_row();
                }
                if elem.is_align_point() {
                    self.is_aligned = true;
                }
            }
            let node = elem.accept(visitor);
            self.stack.push(node);
        }
        self.dump_stack_onto_array();
    }

    pub fn dump_stack_onto_array(&mut self) {
        if self.array.is_empty() {
            self.add_row()
        }
        let nodes = self.stack.iter().map(|n| n.clone().as_array()).flatten().collect();
        self.array.last_mut().unwrap().push(Node::Array(nodes));
        self.stack.clear();
    }

    pub fn add_row(&mut self) {
        self.array.push(Vec::new())
    }

    pub fn convert_flatten(&mut self) -> Node {
        let nodes = self.array.iter().flatten().map(|n| n.clone().as_array()).flatten();
        Node::Array(nodes.collect())
    }

    pub fn convert_leftright_align(&mut self) -> Node {
        let body = self.get_array_body();
        let h_lines_before_row = vec![Vec::new(); self.array.len() + 1];
        let cols = self.get_cols_leftright_align();

        Node::Node(katex::Node::Array(katex::ArrayBuilder::default()
            .body(body)
            .h_lines_before_row(h_lines_before_row)
            .add_jot(true)
            .leqno(false)
            .col_separation_type(katex::ColSeparationType::Align)
            .cols(cols)
            .build().unwrap()))
    }

    pub fn get_array_body(&mut self) -> katex::NodeArray2D {
        self.array.iter().map(|v| v.iter().map(|n| {
            let ordgroup = katex::Node::OrdGroup(katex::OrdGroupBuilder::default().body(n.clone().as_array()).build().unwrap());
            katex::Node::Styling(katex::StylingBuilder::default()
                .style(katex::StyleStr::Display)
                .body([ordgroup].to_vec())
                .build().unwrap()
            )
        }).collect()).collect()
    }

    pub fn get_cols_leftright_align(&mut self) -> Vec<katex::AlignSpec> {
        let mut cols: Vec<katex::AlignSpec> = Vec::new();
        for i in 0..self.count_columns() {
            let align = katex::Align {
                align: if i % 2 == 0 { "r".to_string() } else { "l".to_string() },
                pregap: if i > 1 && i % 2 == 0 { Some(1f32) } else { Some(0f32) },
                postgap: Some(0f32),
            };
            cols.push(katex::AlignSpec::Align(align));
        }
        return cols;
    }

    pub fn count_columns(&mut self) -> usize {
        self.array.iter().map(|row| row.len()).max().unwrap_or(0)
    }

    pub fn get_h_lines_before_row(&mut self) -> Vec<Vec<bool>> {
        vec![Vec::new(); self.array.len() + 1]
    }
}

pub struct VecConverter<'a> {
    pub elem: &'a typst::math::VecElem,
}

// pub struct VecConverter<'a> {
//     pub elem: &'a typst::math::VecElem,
// }

// impl<'a> VecConverter<'a> {
//     pub fn new(elem: &'a typst::math::VecElem) -> Self {
//         Self {
//             elem,
//         }
//     }

//     pub fn convert(&mut self) -> Node {
//         // Left right > Array
//     }
// }

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
