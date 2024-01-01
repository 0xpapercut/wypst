// Mainly for debug purposes

use comemo::Prehashed;
use comemo::Track;
use typst;
use typst_math_transpiler_core::convert;
use typst::World;

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
    fn file(&self, id: typst_syntax::FileId) -> typst::diag::FileResult<typst::foundations::Bytes> {
        unimplemented!();
    }
    fn font(&self, index: usize) -> Option<typst::text::Font> {
        unimplemented!();
    }
    fn main(&self) -> typst_syntax::Source {
        unimplemented!();
    }
    fn packages(&self) -> &[(typst_syntax::PackageSpec,Option<typst::diag::EcoString>)] {
        unimplemented!();
    }
    fn source(&self, id: typst_syntax::FileId) -> typst::diag::FileResult<typst_syntax::Source> {
        unimplemented!();
    }
    fn today(&self, offset: Option<i64>) -> Option<typst::foundations::Datetime> {
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
    let content = eval(&world, "x + y");
    let math: &typst::math::EquationElem = content.to::<typst::math::EquationElem>().unwrap();
    println!("{:#?}", math);
    println!("{:#?}", convert(&content));
}
