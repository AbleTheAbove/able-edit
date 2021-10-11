pub struct PluginsHolder {
    pub list: Vec<Plugin>,
    pub engine: Engine,
}
impl PluginsHolder {
    pub fn load() {
        for x in 0..10 {}
    }
}

pub struct Plugin {
    pub id: String,
    pub source: AST,
}

impl Plugin {
    pub fn test() -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();

        engine.register_fn("add", add);
        let ast = engine.compile_file("./plugins/plugin.rhai".into())?;
        // print the ast for debugging purposes
        println!("{:?}", ast);

        Ok(())
    }
}

fn add(x: INT, y: INT) -> INT {
    x + y
}
use rhai::{Engine, EvalAltResult, AST, INT};
