/// Reference: parseNode.js

use std::collections::HashMap;
use serde::Serialize;
use derive_builder::Builder;
use typst::foundations::Style;
use crate::katex::types::*;
use crate::katex::source::SourceLocation;
use crate::katex::symbol;

pub type NodeArray = Vec<Node>;
pub type NodeArray2D = Vec<Vec<Node>>;

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Node {
    Array(Array),
    CdLabel(CdLabel),
    CdLabelParent(CdLabelParent),
    Color(Color),
    ColorToken(ColorToken),
    Op(Op),
    OrdGroup(OrdGroup),
    Raw(Raw),
    Size(Size),
    Styling(Styling),
    SupSub(SupSub),
    Tag(Tag),
    Text(Text),
    Url(Url),
    Verb(Verb),
    Atom(Atom),
    MathOrd(MathOrd),
    Spacing(Spacing),
    TextOrd(TextOrd),
    AccentToken(AccentToken),
    OpToken(OpToken),
    Accent(Accent),
    AccentUnder(AccentUnder),
    Cr(Cr),
    DelimSizing(DelimSizing),
    Enclose(Enclose),
    Environment(Environment),
    Font(Font),
    GenFrac(GenFrac),
    HBox(HBox),
    HorizBrace(HorizBrace),
    HRef(HRef),
    Html(Html),
    HtmlMathML(HtmlMathML),
    IncludeGraphics(IncludeGraphics),
    Infix(Infix),
    Internal(Internal),
    Kern(Kern),
    Lap(Lap),
    LeftRight(LeftRight),
    LeftRightRight(LeftRightRight),
    MathChoice(MathChoice),
    Middle(Middle),
    MClass(MClass),
    OperatorName(OperatorName),
    Overline(Overline),
    Phantom(Phantom),
    HPhantom(HPhantom),
    VPhantom(VPhantom),
    Pmb(Pmb),
    RaiseBox(RaiseBox),
    Rule(Rule),
    Sizing(Sizing),
    Smash(Smash),
    Sqrt(Sqrt),
    Underline(Underline),
    VCenter(VCenter),
    XArrow(XArrow),
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_value(&self).unwrap())
    }
}

#[derive(Clone, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    #[builder(default = "Mode::Math")]
    pub mode: Mode,
    #[builder(default)]
    pub loc: Option<SourceLocation>,
    #[builder(default)]
    pub col_separation_type: Option<ColSeparationType>,
    #[builder(default)]
    pub hskip_before_and_after: Option<bool>,
    #[builder(default)]
    pub add_jot: Option<bool>,
    #[builder(default)]
    pub cols: Option<Vec<AlignSpec>>,
    #[builder(default = "1.0")]
    pub arraystretch: f32,
    #[builder(default = "Vec::new()")]
    pub body: NodeArray2D,
    #[builder(default = "Vec::new()")]
    pub row_gaps: Vec<Option<Measurement>>,
    #[builder(default = "Vec::new()")]
    pub h_lines_before_row: Vec<Vec<bool>>,
    #[builder(default)]
    pub tags: Option<Vec<TagType>>,
    #[builder(default)]
    pub leqno: Option<bool>,
    #[builder(default)]
    #[serde(rename = "isCD")]
    pub is_cd: Option<bool>,
}

#[derive(Clone, Serialize)]
pub struct CdLabel {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub side: String,
    pub label: Box<Node>,
}

#[derive(Clone, Serialize, Builder)]
pub struct CdLabelParent {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub side: String,
    pub label: Box<Node>,
}

 #[derive(Clone, Serialize)]
pub struct Color {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub color: String,
    pub body: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct ColorToken {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub color: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Op { // TODO Validation
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub limits: bool,
    pub always_handle_sup_sub: Option<bool>,
    pub suppress_base_shift: Option<bool>,
    pub parent_is_sup_sub: bool,
    pub symbol: bool,
    pub name: String,
    pub body: Option<NodeArray>,
}

#[derive(Clone, Serialize, Builder)]
pub struct OrdGroup {
    #[builder(default = "Mode::Math")]
    pub mode: Mode,
    #[builder(default)]
    pub loc: Option<SourceLocation>,
    #[builder(default = "Vec::new()")]
    pub body: NodeArray,
    #[builder(default)]
    pub semisimple: Option<bool>,
}

#[derive(Clone, Serialize)]
pub struct Raw {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub string: String,
}

#[derive(Clone, Serialize)]
pub struct Size {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub value: Measurement,
    pub is_blank: bool,
}

#[derive(Clone, Serialize, Builder)]
pub struct Styling {
    #[builder(default = "Mode::Math")]
    pub mode: Mode,
    #[builder(default)]
    pub loc: Option<SourceLocation>,
    pub style: StyleStr,
    #[builder(default = "Vec::new()")]
    pub body: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct SupSub {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub base: Option<Box<Node>>,
    pub sup: Option<Box<Node>>,
    pub sub: Option<Box<Node>>,
}

#[derive(Clone, Serialize)]
pub struct Tag {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: NodeArray,
    pub tag: NodeArray,
}

#[derive(Clone, Serialize, Builder)]
pub struct Text {
    #[builder(default = "Mode::Math")]
    pub mode: Mode,
    #[builder(default)]
    pub loc: Option<SourceLocation>,
    #[builder(default = "Vec::new()")]
    pub body: NodeArray,
    #[builder(default)]
    pub font: Option<String>
}

#[derive(Clone, Serialize)]
pub struct Url {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub url: String,
}

#[derive(Clone, Serialize)]
pub struct Verb {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: String,
    pub star: bool,
}

#[derive(Clone, Serialize)]
pub struct Atom {
    pub family: symbol::AtomGroup,
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(Clone, Serialize)]
pub struct MathOrd {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(Clone, Serialize)]
pub struct Spacing {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(Clone, Serialize)]
pub struct TextOrd {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(Clone, Serialize)]
pub struct AccentToken {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(Clone, Serialize)]
pub struct OpToken {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub text: String,
}

#[derive(Clone, Serialize)]
pub struct Accent {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub is_stretchy: Option<bool>,
    pub is_shifty: Option<bool>,
    pub base: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct AccentUnder {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub is_stretchy: Option<bool>,
    pub is_shifty: Option<bool>,
    pub base: Box<Node>,
}

#[derive(Clone, Serialize, Builder)]
pub struct Cr {
    #[builder(default = "Mode::Math")]
    pub mode: Mode,
    #[builder(default)]
    pub loc: Option<SourceLocation>,
    #[builder(default = "true")]
    pub new_line: bool,
    #[builder(default)]
    pub size: Option<Measurement>,
}

#[derive(Clone, Serialize)]
pub struct DelimSizing {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub size: SizeType,
    pub mclass: MClassType,
    pub delim: String,
}

#[derive(Clone, Serialize)]
pub struct Enclose {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub body:Box<Node>
}

#[derive(Clone, Serialize)]
pub struct Environment {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub name: String,
    pub name_group: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct Font {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub font: String,
    pub body: Box<Node>,
}

#[derive(Clone, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GenFrac {
    #[builder(default = "Mode::Math")]
    pub mode: Mode,
    #[builder(default)]
    pub loc: Option<SourceLocation>,
    #[builder(default = "false")]
    pub continued: bool,
    pub numer: Box<Node>,
    pub denom: Box<Node>,
    #[builder(default = "true")]
    pub has_bar_line: bool,
    #[builder(default)]
    pub left_delim: Option<String>,
    #[builder(default)]
    pub right_delim: Option<String>,
    #[builder(default = "GenFracSizeType::Auto")]
    pub size: GenFracSizeType,
    #[builder(default)]
    pub bar_size: Option<Measurement>,
}

#[derive(Clone, Serialize)]
pub struct HBox {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct HorizBrace {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub is_over: bool,
    pub base: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct HRef {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub href: String,
    pub body: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct Html {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub attributes: HashMap<String, String>,
    pub body: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct HtmlMathML {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub html: NodeArray,
    pub mathml: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct IncludeGraphics {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub alt: String,
    pub width: Measurement,
    pub height: Measurement,
    pub total_height: Measurement,
    pub src: String,
}

#[derive(Clone, Serialize)]
pub struct Infix {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub replace_with: String,
    pub size: Option<Measurement>,
    pub token: Option<Token>,
}

#[derive(Clone, Serialize)]
pub struct Internal {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
}

#[derive(Clone, Serialize)]
pub struct Kern {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub dimension: Measurement,
}

#[derive(Clone, Serialize)]
pub struct Lap {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub alignment: String,
    pub body: Box<Node>,
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeftRight {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: NodeArray,
    pub left: String,
    pub right: String,
    pub right_color: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct LeftRightRight {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub delim: String,
    pub color: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct MathChoice {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub display: NodeArray,
    pub text: NodeArray,
    pub script: NodeArray,
    pub scriptscript: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct Middle {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub delim: String,
}

#[derive(Clone, Serialize)]
pub struct MClass {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub mclass: String,
    pub body: NodeArray,
    pub is_character_box: bool,
}

#[derive(Clone, Serialize)]
pub struct OperatorName {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: NodeArray,
    pub always_handle_sup_sub: bool,
    pub limits: bool,
    pub parent_is_sup_sub: bool,
}

#[derive(Clone, Serialize)]
pub struct Overline {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct Phantom {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct HPhantom {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct VPhantom {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct Pmb {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub mclass: String,
    pub body: NodeArray,
}

#[derive(Clone, Serialize)]
pub struct RaiseBox {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub dy: Measurement,
    pub body: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct Rule {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub shift: Option<Measurement>,
    pub width: Measurement,
    pub height: Measurement,
}

#[derive(Clone, Serialize)]
pub struct Sizing {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub size: f32,
    pub body: NodeArray
}

#[derive(Clone, Serialize)]
pub struct Smash {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: NodeArray,
    pub smash_height: bool,
    pub smash_depth: bool,
}

#[derive(Clone, Serialize)]
pub struct Sqrt {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<Node>,
    pub index: Option<Box<Node>>,
}

#[derive(Clone, Serialize)]
pub struct Underline {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct VCenter {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub body: Box<Node>,
}

#[derive(Clone, Serialize)]
pub struct XArrow {
    pub mode: Mode,
    pub loc: Option<SourceLocation>,
    pub label: String,
    pub body: Box<Node>,
    pub below: Option<Box<Node>>,
}

impl ArrayBuilder {
    /// Adds a new row to the `Array` node.
    pub fn next_row(&mut self) {
        self.body.as_mut().unwrap().push(Vec::new());
    }

    /// Adds a `Node` to the current row of the `Array` node.
    pub fn push_node(&mut self, node: Node) {
        self.body.as_mut().unwrap().last_mut().unwrap().push(node);
    }

    /// Encloses a `NodeArray` inside a `Styling` node before adding to the current row.
    pub fn push_node_array(&mut self, nodes: NodeArray, mode: Mode, style: StyleStr) {
        let styling = StylingBuilder::default()
            .mode(mode)
            .style(style)
            .body(nodes)
            .build()
            .unwrap();
        let node = Node::Styling(styling);
        self.body.as_mut().unwrap().last_mut().unwrap().push(node);
    }

    pub fn count_columns(&mut self) -> usize {
        return self.body.iter().map(|row| row.len()).max().unwrap_or(0);
    }

    // pub fn build_flattened(&mut self) {
    //     let mut nodes: NodeArray = Vec::new();
    //     for styling in self.body {

    //     }
    // }
}
