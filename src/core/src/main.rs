// Mainly for debug purposes

use comemo::Prehashed;
use comemo::Track;
use comemo::Tracked;
use comemo::TrackedMut;
use typst;
use typst::engine::Engine;
use typst::foundations::NativeElement;
use typst::foundations::PlainText;
use typst::foundations::Repr;
use typst::foundations::Resolve;
use typst_math_transpiler_core::convert;
use typst::eval::Tracer;
use typst::World;
// mod tree;
// use typst_library;
// use typst_library::build;

struct FakeWorld {
    library: Prehashed<typst::Library>,
}

impl FakeWorld {
    fn new() -> Self {
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

fn eval(world: &dyn World, string: &str) -> typst::foundations::Content {
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

pub fn main() {
    let mut world = FakeWorld::new();
    let content = eval(&world, "x + 2"); // tenho que descobrir como printar esse A corretamente.
    // let content = eval(&world, "\"area\" = pi dot \"radius\"^2");
    // let content = eval(&world, "\"ab\"");
    let math: &typst::math::EquationElem = content.to::<typst::math::EquationElem>().unwrap();
    println!("{:#?}", math);
    println!("{:#?}", convert(&content));
    // let text: &typst::text::TextElem = math.body().to::<typst::text::TextElem>().unwrap();
    // let a = text.text().trim_matches('\"');
    // println!("{:#?}", a.chars().count());
    // let math: &typst::math::EquationElem = content.to::<typst::math::EquationElem>().unwrap();
    // let lr = math.body().to::<typst::math::LrElem>().unwrap();
    // let style    s = typst::foundations::Styles::new();
    // let chain = typst::foundations::StyleChain::new(&styles);
    // let height = lr.size(chain).unwrap_or(typst::layout::Rel::one()).resolve(chain);
    // println!("{:?}", height);
    // println!("{:#?}", lr.body());
    // tree::conver

    // println!("{:#?}", math.body().is_sequence());
    // world.track();
    // let mut locator = typst::model::Locator::new();
    // let introspector = typst::model::Introspector::default();
    // let mut delayed = typst::model::DelayedErrors::new();
    // let vt = typst::model::Vt {
    //     world,
    //     introspector: introspector.track(),
    //     locator: &mut locator,
    //     delayed: delayed.track_mut(),
    //     tracer,
    // };

    // let mut vm = typst::eval::Vm::
    // let module = typst_library::math::module();
    // let scope = module.scope();
    // let val = scope.get("lim");
}
