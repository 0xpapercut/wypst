use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde_json;
use typst;

mod converter;
mod katex;
mod utils;
mod node;
mod ext;
mod content;
mod symbol;

#[macro_use]
extern crate derive_builder;


#[wasm_bindgen]
pub fn parse_tree(expression: &str) -> JsValue {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let mut world = utils::FakeWorld::new();
    let content = utils::eval(&world, expression);
    let katex_tree = converter::convert(&content);
    to_value(&katex_tree).unwrap()
}

pub fn convert(content: &typst::foundations::Content) -> serde_json::Value {
    let katex_tree = converter::convert(content);
    serde_json::to_value(&katex_tree).unwrap()
}

#[cfg(debug_assertions)]
#[wasm_bindgen]
pub fn typst_tree_str(text: &str) -> JsValue {
    let root = typst::syntax::parse_math(text);
    format!("{:#?}", root).into()
}
