use serde::Serialize;
use crate::katex::NodeArray;

use super::SourceLocation;

#[derive(Clone, Serialize, PartialEq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Math,
    Text,
}

// Refernece: array.js
#[derive(Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ColSeparationType { // TODO
    AlignAt,
    Gather,
    Small,
    #[serde(rename = "CD")]
    CD,
}

// Reference: array.js
#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AlignSpec {
    Separator(Separator),
    Align(Align),
}

#[derive(Clone, Serialize)]
pub struct Separator {
    separator: String,
}

#[derive(Clone, Serialize)]
pub struct Align {
    align: String,
    pregap: Option<f32>,
    postgap: Option<f32>,
}

// Reference: units.js
#[derive(Clone, Serialize)]
pub struct Measurement {
    number: f32,
    unit: String,
}

#[derive(Clone, Serialize)]
pub enum TagType {
    Bool(bool),
    NodeArray(NodeArray),
}

// Reference: types.js
#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleStr {
    Text,
    Display,
    Script,
    ScriptScript,
}

#[derive(Clone, Serialize)]
pub enum SizeType { // TODO: Check serialization
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

#[derive(Clone, Serialize)]
pub enum MClassType {
    MOpen,
    MClose,
    MRel,
    MOrd,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GenFracSizeType {
    StyleStr(StyleStr),
    Auto,
}

// Reference: Token.js
#[derive(Clone, Serialize)]
pub struct Token {
    text: String,
    loc: Option<SourceLocation>,
    noexpand: Option<bool>,
    treat_as_relax: Option<bool>,
}
