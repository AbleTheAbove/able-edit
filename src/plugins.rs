pub struct PluginsHolder {
    pub list: Vec<Plugin>,
    pub engine: Engine,
}
impl PluginsHolder {
    pub fn load() {}
}

pub struct Plugin {
    pub id: String,
    pub source: AST,
}

impl Plugin {
    pub fn test() -> Result<(), Box<EvalAltResult>> {
        // TODO: Move to the PluginsHolder
        let mut engine = Engine::new();

        engine.register_fn("register_command", register_command);

        let ast = engine.compile_file("./plugins/example.ashell".into())?;
        // print the ast for debugging purposes
        // println!("{:?}", ast);

        let mut value: Dynamic = 1_i64.into();
        let mut scope = Scope::new();
        let result = engine.call_fn_dynamic(
            &mut scope,
            &ast,
            false,
            "action",
            Some(&mut value), // binding the 'this' pointer
            [41_i64.into()],
        )?;

        println!("{:?}", value.as_int());
        Ok(())
    }
}

pub fn register_command(name: String) {
    // Register a rhai command for use by ashell
    println!("{}", name);
}

use rhai::{Dynamic, Engine, EvalAltResult, Func, Scope, AST, INT};
