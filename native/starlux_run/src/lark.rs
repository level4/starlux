#[macro_use]
use starlark::starlark_module; 
use anyhow::Result;
use starlark::any::ProvidesStaticType;
use starlark::environment::FrozenModule;
use starlark::environment::Globals;
use starlark::environment::GlobalsBuilder;
use starlark::environment::Module;
use starlark::eval::Evaluator;
use starlark::eval::ReturnFileLoader;
use starlark::syntax::AstModule;
use starlark::syntax::Dialect;
use starlark::values::none::NoneType;
use starlark::values::Value;

use std::cell::RefCell;
use std::fs;
use std::env;
use serde_json;
use std::fs::File;
use std::io::Write;
// use std::path;
use std::path::Path;
// use std::path::PathBuf;

// Define a store in which to accumulate JSON strings
#[derive(Debug, ProvidesStaticType, Default)]
struct Store(RefCell<Vec<String>>);
// struct Store(RefCell<Vec<serde_json::Value>>);

impl Store {
    fn add(&self, x: String) {
        self.0.borrow_mut().push(x)
    }
    // fn add(&self, x: serde_json::Value) {
    //     self.0.borrow_mut().push(x);
    // }
}
#[starlark_module]
pub fn starlark_emit(builder: &mut GlobalsBuilder) {
    fn emit(x: Value, eval: &mut Evaluator) -> anyhow::Result<NoneType> {
        // We modify extra (which we know is a Store) and add the JSON of the
        // value the user gave.
        eval.extra
            .unwrap()
            .downcast_ref::<Store>()
            .unwrap()
            .add(x.to_json()?);
        Ok(NoneType)
    }
}
fn get_file(path: &str) -> Result<String> {
    let path: &Path = Path::new(path);
    println!("Path: {:?}", path);

    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn get_source(file: &str) -> &str {
    match get_file(file) {
        Ok(content) => Box::leak(content.into_boxed_str()),
        Err(_) => panic!("Failed to read file"),
    }
}

pub fn get_module(file: &str) -> starlark::Result<FrozenModule> {
    let ast = AstModule::parse(file, get_source(file).to_owned(), &Dialect::Extended)?;

    // We can get the loaded modules from `ast.loads`.
    // And ultimately produce a `loader` capable of giving those modules to Starlark.
    let mut loads = Vec::new();
    for load in ast.loads() {
        println!("Load: {:?}", load);
        loads.push((load.module_id.to_owned(), get_module(load.module_id)?));
    }
    let modules = loads.iter().map(|(a, b)| (a.as_str(), b)).collect();
    let mut loader = ReturnFileLoader { modules: &modules };

    let globals = GlobalsBuilder::standard().with(starlark_emit).build();
    let module = Module::new();
    let store = Store::default();
    {
        let mut eval = Evaluator::new(&module);
        eval.set_loader(&mut loader);
        eval.extra = Some(&store);
        let cool = eval.eval_module(ast, &globals)?;
        let heap = module.heap();
        println!("Result: {:?}", cool);

        let res = eval.eval_function(
            cool,
            // &[heap.alloc(4), heap.alloc(2), heap.alloc(1)],
            &[heap.alloc(8)],
            // &[("yo", heap.alloc(8))],
            &[],

        )?;
        println!("Result: {:?}", res.to_json()?);
        let mut file = File::create("output.json").unwrap();
        let json = serde_json::to_string(&*store.0.borrow()).unwrap();
        writeln!(file, "{}", json).unwrap();
    }
    println!("Result: {:?}", store);
    // After creating a module we freeze it, preventing further mutation.
    // It can now be used as the input for other Starlark modules.
    Ok(module.freeze()?)
}

fn print_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument passed is: {}", args[1]);
    } else {
        println!("No arguments were passed.");
    }
}



pub fn evaluate_starlark_code(code: &str) -> Result<(String, String), starlark::Error> {
        let ast = AstModule::parse("input.star", code.to_owned(), &Dialect::Extended)?;
    let globals = GlobalsBuilder::standard().with(starlark_emit).build();
    let module = Module::new();
    let store = Store::default();

    // Evaluate the Starlark code and capture the result
    let eval_result: Value;
    {
        let mut eval = Evaluator::new(&module);
        eval.extra = Some(&store);
        eval_result = eval.eval_module(ast, &globals)?;
    }

    // Serialize the eval_result to a string, if needed
    let eval_str = eval_result.to_str();

    // Serialize the store's contents to JSON
    let store_json = serde_json::to_string(&*store.0.borrow()).unwrap();
    
    Ok((eval_str, store_json))
}
