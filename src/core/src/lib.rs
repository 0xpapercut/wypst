use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use serde_json;
use typst;

mod tree;
mod katex;
mod utils;
mod node;

#[wasm_bindgen]
pub fn parse_tree(expression: &str) -> JsValue {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let mut world = utils::FakeWorld::new();
    let content = utils::eval(&world, expression);
    // let math: &typst::math::EquationElem = content.to::<typst::math::EquationElem>().unwrap();
    // let katex_tree = tree::convert(math.body());
    let katex_tree = tree::convert(&content);
    // let typst_tree = typst::syntax::parse_math(expression);
    // let katex_tree = tree::convert_expr(typst_tree.cast().unwrap());
    to_value(&katex_tree).unwrap()
}

pub fn convert(content: &typst::foundations::Content) -> serde_json::Value {
    let katex_tree = tree::convert(content);
    serde_json::to_value(&katex_tree).unwrap()
}

// pub fn parse_tree_json(expression: &str) -> serde_json::Value {
//     let typst_tree = typst::syntax::parse_math(expression);
//     let katex_tree = tree::convert(typst_tree.cast().unwrap());
//     serde_json::to_value(&katex_tree).unwrap()
// }

#[cfg(debug_assertions)]
#[wasm_bindgen]
pub fn typst_tree_str(text: &str) -> JsValue {
    let root = typst::syntax::parse_math(text);
    format!("{:#?}", root).into()
}
