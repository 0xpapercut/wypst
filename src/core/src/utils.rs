use wasm_bindgen::prelude::*;
use typst::syntax::ast::Expr;
use typst::foundations::{Value, Scope};
use typst::symbols::Symbol;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[macro_export]
macro_rules! qlog {
    ($val:expr) => {
        log(&format!("{:?}", $val))
    };
}

use comemo::Prehashed;
use comemo::Track;
use comemo::Tracked;
use comemo::TrackedMut;
use typst;
use typst::engine::Engine;
use typst::foundations::Resolve;
use crate::convert;
// use typst_math_transpiler_core::convert;
use typst::eval::Tracer;
use typst::World;
// mod tree;
// use typst_library;
// use typst_library::build;

pub struct FakeWorld {
    library: Prehashed<typst::Library>,
}

impl FakeWorld {
    pub fn new() -> Self {
        FakeWorld {
            library: Prehashed::new(typst::Library::build()),
        }
    }
}

impl World for FakeWorld {
    fn library(&self) -> &Prehashed<typst::Library> {
        &self.library
    }
    fn book(&self) ->  &Prehashed<typst::text::FontBook> {
        unimplemented!();
    }
    fn file(&self,id:typst_syntax::FileId) -> typst::diag::FileResult<typst::foundations::Bytes> {
        unimplemented!();
    }
    fn font(&self,index:usize) -> Option<typst::text::Font> {
        unimplemented!();
    }
    fn main(&self) -> typst_syntax::Source {
        unimplemented!();
    }
    fn packages(&self) ->  &[(typst_syntax::PackageSpec,Option<typst::diag::EcoString>)] {
        unimplemented!();
    }
    fn source(&self,id:typst_syntax::FileId) -> typst::diag::FileResult<typst_syntax::Source> {
        unimplemented!();
    }
    fn today(&self,offset:Option<i64>) -> Option<typst::foundations::Datetime> {
        unimplemented!();
    }
}

pub fn eval(world: &dyn World, string: &str) -> typst::foundations::Content {
    // Make engine
    let introspector = typst::introspection::Introspector::default();
    let mut locator = typst::introspection::Locator::default();
    let mut tracer = typst::eval::Tracer::default();

    let engine = typst::engine::Engine {
        world: world.track(),
        introspector: introspector.track(),
        route: typst::engine::Route::default(),
        locator: &mut locator,
        tracer: tracer.track_mut(),
    };

    // // Make context
    // let styles = typst::foundations::Styles::new();
    // let chain = typst::foundations::StyleChain::new(&styles);
    // let regions = typst::layout::Regions::one(, expand)
    // let ctx = typst::math::MathContext::new(engine, chain, )

    let result = typst::eval::eval_string(
        world.track(),
        string,
        typst::syntax::Span::detached(),
        typst::eval::EvalMode::Math,
        world.library().math.scope().clone()
    ).unwrap();
    match result {
        typst::foundations::Value::Content(content) => content,
        _ => panic!(),
    }
}

// pub fn get_symbol_from_expr(expr: Expr, scope: &Scope) -> Result<Symbol, String> {
//     match expr {
//         Expr::FieldAccess(field_access) => {
//             let symbol = get_symbol_from_expr(field_access.target(), scope)?;
//             symbol.modified(field_access.field().as_str()).map_err(|_| String::from("Failed to modify symbol"))
//         },
//         Expr::MathIdent(expr) => {
//             let value = scope.get(expr.as_str()).ok_or("Failed to get value from scope")?;
//             match value {
//                 Value::Symbol(symbol) => {
//                     println!("Indeed it is a symbol: {:#?}", symbol);
//                     Ok(symbol.clone())
//                 },
//                 Value::Content(content) => {
//                     println!("Will try to print content...");
//                     println!("This is content: {:#?}", content.label().unwrap());
//                     // log!("This is content: {:#?}", content);
//                     panic!()
//                 }
//                 _ => Err(format!("Value {:#?} is not a symbol", value.ty())),
//             }
//         },
//         _ => Err(String::from("Unsupported expression type")),
//     }
// }
