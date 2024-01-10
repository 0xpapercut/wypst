use typst;
use typst::foundations::Content;

use crate::katex;
use crate::node::*;
use crate::content::*;
use crate::ext::*;
use crate::utils::insert_separator;

pub fn convert(root: &Content) -> Node {
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
    fn visit_equation(&mut self, content: &Content) -> Node {
        let elem = content.to_equation();
        Node::Array(elem.body().accept(self).as_array())
    }

    fn visit_op(&mut self, content: &Content) -> Node {
        let elem = content.to_op();

        let _text = elem.text();
        let _limits = elem.limits(self.styles);

        let node = katex::OpBuilder::default()
            .limits(_limits)
            .parent_is_sup_sub(false)
            .symbol(false)
            .name(format!("\\{}", _text.plain_text()))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_mat(&mut self, content: &Content) -> Node {
        let elem = content.to_mat();
        let mut constructor = katex::ArrayConstructor::default();

        for row in elem.rows() {
            constructor.next_row();
            for content in row {
                let node = content.accept(self);
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.into_array())
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

    fn visit_vec(&mut self, content: &Content) -> Node {
        let mut converter = VecConverter::new(content.to_vec());
        converter.convert(self)
    }

    fn visit_frac(&mut self, content: &Content) -> Node {
        let elem = content.to_frac();

        let _num = elem.num();
        let _denom = elem.denom();

        let numer = katex::OrdGroupBuilder::default()
            .body(_num.accept(self).into_array())
            .build().unwrap().into_node();

        let denom = katex::OrdGroupBuilder::default()
            .body(_denom.accept(self).into_array())
            .build().unwrap().into_node();

        let node = katex::GenFracBuilder::default()
            .numer(Box::new(numer))
            .denom(Box::new(denom))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_align_point(&mut self, content: &Content) -> Node {
        let node = katex::OrdGroupBuilder::default().build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_linebreak(&mut self, content: &Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_sequence(&mut self, content: &Content) -> Node {
        let mut converter = SequenceConverter::new(content);
        converter.convert(self)
    }

    fn visit_space(&mut self, content: &Content) -> Node {
        Node::Array(Vec::new())
    }

    fn visit_text(&mut self, content: &Content) -> Node {
        let mut text_converter = TextConverter::new(content.to_text());
        text_converter.convert()
    }

    fn visit_lr(&mut self, content: &Content) -> Node {
        let elem = content.to_lr();

        let _body = elem.body();
        let _size = elem.size(self.styles); // unsupported

        let mut body = _body.accept(self).into_array();
        let left = match body.remove(0) {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };
        let right = match body.pop().unwrap() {
            katex::Node::Atom(atom) => atom.text,
            _ => panic!("Not an atom!"),
        };

        let node = katex::LeftRightBuilder::default()
            .body(body)
            .left(left)
            .right(right)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_attach(&mut self, content: &Content) -> Node {
        let elem = content.to_attach();

        let _base = elem.base();
        let _t = elem.t(self.styles);
        let _b = elem.b(self.styles);
        let _tl = elem.tl(self.styles); // unsupported
        let _bl = elem.bl(self.styles); // unsupported
        let _tr = elem.tr(self.styles); // unsupported
        let _br = elem.br(self.styles); // unsupported
        if _tl.is_some() { warn!("Top left element is unsupported."); }
        if _tr.is_some() { warn!("Top right element is unsupported."); }
        if _bl.is_some() { warn!("Bottom left element is unsupported."); }
        if _br.is_some() { warn!("Bottom right element is unsupported."); }

        let base = _base.accept(self).into_node_fallback_ordgroup(katex::Mode::Math);
        let sup = _t.map(|c| c.accept(self)).map(|n| n.into_node_fallback_ordgroup(katex::Mode::Math));
        let sub = _b.map(|c| c.accept(self)).map(|n| n.into_node_fallback_ordgroup(katex::Mode::Math));

        let node = katex::SupSubBuilder::default()
            .base(Some(base).map(Box::new))
            .sup(sup.map(Box::new))
            .sub(sub.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_math_style(&mut self, content: &Content) -> Node {
        let elem = content.to_math_style();

        let _body = elem.body();
        let _variant = elem.variant(self.styles);
        let _bold = elem.bold(self.styles); // unsupported
        let _italic = elem.italic(self.styles); // unsupported
        let _size = elem.size(self.styles); // unsupported
        let _cramped = elem.cramped(self.styles); // unsupported
        if _bold.is_some() { warn!("Bold options are unsupported."); }
        if _italic.is_some() { warn!("Italic options are unsupported."); }
        if _size.is_some() { warn!("Size options are unsupported."); }
        if _cramped.is_some() { warn!("Cramped options are unsupported."); }

        let body = _body.accept(self).into_node().unwrap();
        let font = match _variant {
            Some(typst::math::MathVariant::Bb) => "mathbb",
            Some(typst::math::MathVariant::Cal) => "mathcal",
            Some(typst::math::MathVariant::Serif) => unimplemented!(),
            Some(typst::math::MathVariant::Sans) => unimplemented!(),
            Some(typst::math::MathVariant::Frak) => unimplemented!(),
            Some(typst::math::MathVariant::Mono) => unimplemented!(),
            None => "mathrm"
        }.to_string();

        let node = katex::FontBuilder::default()
            .body(Box::new(body))
            .font(font)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_binom(&mut self, content: &Content) -> Node {
        let elem = content.to_binom();

        let _upper = elem.upper();
        let _lower = elem.lower();

        let numer = katex::OrdGroupBuilder::default()
            .body(_upper.accept(self).into_array())
            .build().unwrap().into_node();

        let separator = katex::Symbol::get(katex::Mode::Math, ',').create_node();
        let denom_body_parts: Vec<katex::NodeArray> = elem.lower().iter().map(|c| c.accept(self).into_array()).collect();
        let denom_body = insert_separator(&denom_body_parts, [separator].to_vec()).iter().flatten().cloned().collect();
        let denom = katex::OrdGroupBuilder::default()
            .body(denom_body)
            .build().unwrap().into_node();

        let node = katex::GenFracBuilder::default()
            .continued(false)
            .numer(numer)
            .denom(denom)
            .has_bar_line(false)
            .left_delim("(".to_string())
            .right_delim(")".to_string())
            .size(katex::GenFracSizeType::Auto)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_cancel(&mut self, content: &Content) -> Node {
        let elem = content.to_cancel();

        let _body = elem.body();
        let _length = elem.length(self.styles); // unsupported
        let _inverted = elem.inverted(self.styles); // unsupported
        let _cross = elem.cross(self.styles); // unsupported
        let _angle = elem.angle(self.styles); // unsupported
        let _stroke = elem.stroke(self.styles); // unsupported

        let body = katex::OrdGroupBuilder::default()
            .body(_body.accept(self).into_array())
            .build().unwrap().into_node();

        let node = katex::EncloseBuilder::default()
            .label("\\cancel")
            .body(body)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_cases(&mut self, content: &Content) -> Node {
        let mut converter = CasesConverter::new(content.to_cases());
        converter.convert(self)
    }

    fn visit_limits(&mut self, content: &Content) -> Node {
        let elem = content.to_limits();

        let _body = elem.body();
        let _inline = elem.inline(self.styles); // unsupported

        // This comes inside an AttachElem, so we have to transform this into an operator
        let node = katex::OpBuilder::default()
            .mode(katex::Mode::Math)
            .limits(true)
            .parent_is_sup_sub(false)
            .symbol(false)
            .body(_body.accept(self).into_array())
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_scripts(&mut self, content: &Content) -> Node {
        let elem = content.to_scripts();

        let _body = elem.body();

        let node = katex::OpBuilder::default()
            .mode(katex::Mode::Math)
            .limits(false)
            .parent_is_sup_sub(false)
            .symbol(false)
            .body(_body.accept(self).into_array())
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_mid(&mut self, content: &Content) -> Node {
        let elem = content.to_mid();

        let _body = elem.body();

        let delim = _body.plain_text().to_string();
        let node = katex::MiddleBuilder::default()
            .delim(delim)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_overbrace(&mut self, content: &Content) -> Node {
        let elem = content.to_overbrace();

        let _body = elem.body();
        let _annotation = elem.annotation(self.styles);

        let base = katex::HorizBraceBuilder::default()
            .label("\\overbrace".to_string())
            .is_over(true)
            .base(_body.accept(self).into_ordgroup(katex::Mode::Math).into_node())
            .build().unwrap().into_node();
        let sup = _annotation.map(|c| c.accept(self).into_ordgroup(katex::Mode::Math).into_node());

        let node = katex::SupSubBuilder::default()
            .base(Box::new(base))
            .sup(sup.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_overline(&mut self, content: &Content) -> Node {
        let elem = content.to_overline();

        let _body = elem.body();

        let body = _body.accept(self).into_ordgroup(katex::Mode::Math).into_node();
        let node = katex::OverlineBuilder::default()
            .body(Box::new(body))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_root(&mut self, content: &Content) -> Node {
        let elem = content.to_root();

        let _index = elem.index(self.styles);
        let _radicand = elem.radicand();

        let index = _index.map(|c| c.accept(self).into_ordgroup(katex::Mode::Math).into_node());
        let body = _radicand.accept(self).into_ordgroup(katex::Mode::Math).into_node();

        let node = katex::SqrtBuilder::default()
            .body(Box::new(body))
            .index(index.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_underbrace(&mut self, content: &Content) -> Node {
        let elem = content.to_underbrace();

        let _body = elem.body();
        let _annotation = elem.annotation(self.styles);

        let base = katex::HorizBraceBuilder::default()
            .base(_body.accept(self).into_ordgroup(katex::Mode::Math).into_node())
            .is_over(false)
            .label("\\underbrace".to_string())
            .build().unwrap().into_node();
        let sub = _annotation.map(|c| c.accept(self).into_ordgroup(katex::Mode::Math).into_node());

        let node = katex::SupSubBuilder::default()
            .base(Box::new(base))
            .sub(sub.map(Box::new))
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_underline(&mut self, content: &Content) -> Node {
        let elem = content.to_underline();

        let _body = elem.body();

        let node = katex::UnderlineBuilder::default()
            .body(_body.accept(self).into_ordgroup(katex::Mode::Math).into_node())
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_h(&mut self, content: &Content) -> Node {
        let elem = content.to_h();

        let _amount = elem.amount();
        let _weak = elem.weak(self.styles); // unsupported

        let length = match _amount {
            typst::layout::Spacing::Fr(fr) => unimplemented!(),
            typst::layout::Spacing::Rel(rel) => rel.abs,
        };

        let node = katex::KernBuilder::default()
            .dimension(katex::Measurement {
                number: length.em.get() as f32,
                unit: "em".to_string(),
            })
            .mode(katex::Mode::Math)
            .build().unwrap().into_node();
        Node::Node(node)
    }

    fn visit_underbracket(&mut self, content: &Content) -> Node {
        // unsupported
        unimplemented!()
    }

    fn visit_overbracket(&mut self, content: &Content) -> Node {
        // unsupported
        unimplemented!()
    }

    fn visit_class(&mut self, content: &Content) -> Node {
        // unsupported
        unimplemented!()
    }

    fn visit_primes(&mut self, content: &Content) -> Node {
        // unsupported
        unimplemented!()
    }

    fn visit_accent(&mut self, content: &Content) -> Node {
        // unsupported
        unimplemented!()
    }
}

pub struct SequenceConverter<'a> {
    pub content: &'a Content,
    pub body: Vec<Vec<Node>>,
    pub stack: Vec<Node>,
    pub is_aligned: bool,
}

impl<'a> SequenceConverter<'a> {
    pub fn new(content: &'a Content) -> Self {
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
        let nodes = self.body.iter().flatten().map(|n| n.clone().into_array()).flatten();
        Node::Array(nodes.collect())
    }

    pub fn convert_align(&mut self) -> Node {
        let mut constructor = katex::ArrayConstructor::default();

        for row in self.body.iter_mut() {
            constructor.next_row();
            for node in row.iter_mut() {
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.clone().into_array())
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

pub struct CasesConverter<'a> {
    pub elem: &'a typst::math::CasesElem,
    pub body: Vec<Vec<Node>>,
    pub stack: Vec<Node>,
}

impl<'a> CasesConverter<'a> {
    pub fn new(elem: &'a typst::math::CasesElem) -> Self {
        Self {
            elem,
            body: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn convert(&mut self, visitor: &mut ContentConverter) -> Node {
        self.process_children(visitor);

        let mut constructor = katex::ArrayConstructor::default();
        for row in self.body.iter_mut() {
            constructor.next_row();
            for node in row {
                let ordgroup = katex::OrdGroupBuilder::default()
                    .body(node.clone().as_array())
                    .build().unwrap().into_node();
                let styling = katex::StylingBuilder::default()
                    .style(katex::StyleStr::Text)
                    .body([ordgroup].to_vec())
                    .build().unwrap().into_node();
                constructor.push_node(styling);
            }
        }

        let cols = vec![
            katex::AlignSpec::Align(katex::Align {
                align: "l".to_string(),
                pregap: Some(0f32),
                postgap: Some(1f32),
            }),
            katex::AlignSpec::Align(katex::Align {
                align: "l".to_string(),
                pregap: Some(0f32),
                postgap: Some(0f32),
            }),
        ];
        let array = constructor.builder()
            .arraystretch(1.2)
            .cols(cols)
            .build().unwrap().into_node();
        let leftright = katex::LeftRightBuilder::default()
            .body([array].to_vec())
            .left("\\{".to_string())
            .right(".".to_string())
            .build().unwrap().into_node();

        Node::Node(leftright)
    }

    pub fn process_children(&mut self, visitor: &mut ContentConverter) {
        for child in self.elem.children() {
            if child.is_sequence() {
                let mut converter = SequenceConverter::new(child);
                converter.process_sequence_elements(visitor);
                self.body.extend(converter.body);
            } else {
                
            }
        }
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
