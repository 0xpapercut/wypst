use crate::node::*;

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

pub trait ContentType {
    fn is_equation(&self) -> bool;
    fn is_space(&self) -> bool;
    fn is_text(&self) -> bool;
    fn is_lr(&self) -> bool;
    fn is_attach(&self) -> bool;
    fn is_math_style(&self) -> bool;
    fn is_h(&self) -> bool;
    fn is_linebreak(&self) -> bool;
    fn is_align_point(&self) -> bool;
    fn is_frac(&self) -> bool;
    fn is_vec(&self) -> bool;
    fn is_mat(&self) -> bool;
    fn is_op(&self) -> bool;

    fn to_equation(&self) -> &typst::math::EquationElem;
    fn to_space(&self) -> &typst::text::SpaceElem;
    fn to_text(&self) -> &typst::text::TextElem;
    fn to_lr(&self) -> &typst::math::LrElem;
    fn to_attach(&self) -> &typst::math::AttachElem;
    fn to_math_style(&self) -> &typst::math::MathStyleElem;
    fn to_h(&self) -> &typst::layout::HElem;
    fn to_linebreak(&self) -> &typst::text::LinebreakElem;
    fn to_align_point(&self) -> &typst::math::AlignPointElem;
    fn to_frac(&self) -> &typst::math::FracElem;
    fn to_vec(&self) -> &typst::math::VecElem;
    fn to_mat(&self) -> &typst::math::MatElem;
    fn to_op(&self) -> &typst::math::OpElem;
}

impl ContentType for typst::foundations::Content {
    fn is_equation(&self) -> bool {
        self.is::<typst::math::EquationElem>()
    }
    fn is_space(&self) -> bool {
        self.is::<typst::text::SpaceElem>()
    }
    fn is_text(&self) -> bool {
        self.is::<typst::text::TextElem>()
    }
    fn is_lr(&self) -> bool {
        self.is::<typst::math::LrElem>()
    }
    fn is_attach(&self) -> bool {
        self.is::<typst::math::AttachElem>()
    }
    fn is_math_style(&self) -> bool {
        self.is::<typst::math::MathStyleElem>()
    }
    fn is_h(&self) -> bool {
        self.is::<typst::layout::HElem>()
    }
    fn is_linebreak(&self) -> bool {
        self.is::<typst::text::LinebreakElem>()
    }
    fn is_align_point(&self) -> bool {
        self.is::<typst::math::AlignPointElem>()
    }
    fn is_frac(&self) -> bool {
        self.is::<typst::math::FracElem>()
    }
    fn is_vec(&self) -> bool {
        self.is::<typst::math::VecElem>()
    }
    fn is_mat(&self) -> bool {
        self.is::<typst::math::MatElem>()
    }
    fn is_op(&self) -> bool {
        self.is::<typst::math::OpElem>()
    }

    fn to_equation(&self) -> &typst::math::EquationElem {
        self.to::<typst::math::EquationElem>().unwrap()
    }
    fn to_space(&self) -> &typst::text::SpaceElem {
        self.to::<typst::text::SpaceElem>().unwrap()
    }
    fn to_text(&self) -> &typst::text::TextElem {
        self.to::<typst::text::TextElem>().unwrap()
    }
    fn to_lr(&self) -> &typst::math::LrElem {
        self.to::<typst::math::LrElem>().unwrap()
    }
    fn to_attach(&self) -> &typst::math::AttachElem {
        self.to::<typst::math::AttachElem>().unwrap()
    }
    fn to_math_style(&self) -> &typst::math::MathStyleElem {
        self.to::<typst::math::MathStyleElem>().unwrap()
    }
    fn to_h(&self) -> &typst::layout::HElem {
        self.to::<typst::layout::HElem>().unwrap()
    }
    fn to_linebreak(&self) -> &typst::text::LinebreakElem {
        self.to::<typst::text::LinebreakElem>().unwrap()
    }
    fn to_align_point(&self) -> &typst::math::AlignPointElem {
        self.to::<typst::math::AlignPointElem>().unwrap()
    }
    fn to_frac(&self) -> &typst::math::FracElem {
        self.to::<typst::math::FracElem>().unwrap()
    }
    fn to_vec(&self) -> &typst::math::VecElem {
        self.to::<typst::math::VecElem>().unwrap()
    }
    fn to_mat(&self) -> &typst::math::MatElem {
        self.to::<typst::math::MatElem>().unwrap()
    }
    fn to_op(&self) -> &typst::math::OpElem {
        self.to::<typst::math::OpElem>().unwrap()
    }
}

pub trait ContentExt {
    fn accept(&self, visitor: &mut dyn ContentVisitor) -> Node;
}

impl ContentExt for typst::foundations::Content {
    fn accept(&self, visitor: &mut dyn ContentVisitor) -> Node {
        match self {
            _ if self.is_equation() => visitor.visit_equation(self),
            _ if self.is_space() => visitor.visit_space(self),
            _ if self.is_text() => visitor.visit_text(self),
            _ if self.is_lr() => visitor.visit_lr(self),
            _ if self.is_attach() => visitor.visit_attach(self),
            _ if self.is_math_style() => visitor.visit_math_style(self),
            _ if self.is_h() => visitor.visit_h(self),
            _ if self.is_linebreak() => visitor.visit_linebreak(self),
            _ if self.is_align_point() => visitor.visit_align_point(self),
            _ if self.is_frac() => visitor.visit_frac(self),
            _ if self.is_vec() => visitor.visit_vec(self),
            _ if self.is_mat() => visitor.visit_mat(self),
            _ if self.is_op() => visitor.visit_op(self),
            _ if self.is_sequence() => visitor.visit_sequence(self),
            _ => panic!("Content element `{:#?}` not implemented yet.", self),
        }
    }
}
