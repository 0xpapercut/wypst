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

fn content_tree(expression: &str) -> Result<typst::foundations::Content, String> {
    let world = utils::FakeWorld::new();
    utils::eval(&world, expression)
}

pub fn convert(content: &typst::foundations::Content) -> serde_json::Value {
    let katex_tree = converter::convert(content);
    serde_json::to_value(&katex_tree).unwrap()
}

#[wasm_bindgen(js_name = "parseTree")]
pub fn parse_tree(expression: &str) -> Result<JsValue, String> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let content = content_tree(expression);
    content.map(|c| {
        let katex_tree = converter::convert(&c);
        to_value(&katex_tree).unwrap()
    })
}

#[wasm_bindgen(js_name = "typstContentTree")]
pub fn typst_content_tree(expression: &str) -> Result<String, String> {
    let content = content_tree(expression);
    match content {
        Ok(tree) => Ok(format!("{:#?}", tree).into()),
        Err(err) => Err(err),
    }
}
