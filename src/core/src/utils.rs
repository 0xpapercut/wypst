use comemo::Prehashed;
use comemo::Track;
use typst;
use typst::World;

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

pub fn eval(world: &dyn typst::World, string: &str) -> Result<typst::foundations::Content, String> {
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
    );

    match result {
        Ok(value) => match value {
            typst::foundations::Value::Content(content) => Ok(content),
            _ => Err("Expected content result.".to_string()),
        }
        Err(err) => Err(err[0].message.to_string())
    }
}

pub fn insert_separator<T: Clone>(list: &[T], separator: T) -> Vec<T> {
    list.iter()
        .flat_map(|x| vec![x.clone(), separator.clone()])
        .take(list.len() * 2 - 1)
        .collect()
}
