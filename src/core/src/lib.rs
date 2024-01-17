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

fn content_tree(expression: &str) -> typst::foundations::Content {
    let world = utils::FakeWorld::new();
    utils::eval(&world, expression)
}

pub fn convert(content: &typst::foundations::Content) -> serde_json::Value {
    let katex_tree = converter::convert(content);
    serde_json::to_value(&katex_tree).unwrap()
}

#[wasm_bindgen(js_name = "parseTree")]
pub fn parse_tree(expression: &str) -> JsValue {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let content = content_tree(expression);
    let katex_tree = converter::convert(&content);
    to_value(&katex_tree).unwrap()
}

#[cfg(debug_assertions)]
#[wasm_bindgen(js_name = "typstContentTree")]
pub fn typst_content_tree(expression: &str) -> String {
    let content = content_tree(expression);
    format!("{:#?}", content).into()
}
