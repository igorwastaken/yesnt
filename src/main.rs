mod functions;
mod module;
use clap::{Arg, Command};
use module::reading::Module;
use std::{
    collections::HashMap,
    io::{self, Write},
};
use v8;
struct Runtime<'s, 'i> {
    context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
    module_cache: HashMap<String, v8::Global<v8::Value>>,
}
/*
fn import_module_callback<'a>(
    scope: &'a mut v8::HandleScope<'a>,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let module_name = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    let runtime_ptr = scope.get_data(0);
    let runtime: &mut Runtime = unsafe { &mut *(runtime_ptr as *mut Runtime) };

    // First mutable borrow ends before starting a new one
    let module = {
        let mut_scope = &mut *scope;
        runtime.import_module(mut_scope, &module_name)
    };

    if let Some(module) = module {
        rv.set(module);
    } else {
        // Create a new string with a separate borrow
        let msg = v8::String::new(scope, "Module not found").unwrap();
        let error = v8::Exception::error(scope, msg);
        scope.throw_exception(error);
    }
}*/

#[allow(unused_variables)]
impl<'s, 'i> Runtime<'s, 'i>
where
    's: 'i,
{
    pub fn new(isolate_scope: &'i mut v8::HandleScope<'s, ()>) -> Self {
        let global = v8::ObjectTemplate::new(isolate_scope);

        // functions
        let printout_fn = v8::FunctionTemplate::new(isolate_scope, functions::printout::printout);
        let key = v8::String::new(isolate_scope, "printout").unwrap();
        global.set(key.into(), printout_fn.into());
        let read_file_fn =
            v8::FunctionTemplate::new(isolate_scope, functions::filesystem::read_file::read_file);
        let key = v8::String::new(isolate_scope, "readFile").unwrap();
        global.set(key.into(), read_file_fn.into());
        let http_get_fn =
            v8::FunctionTemplate::new(isolate_scope, functions::requests::http::http_get);
        let http_get_key = v8::String::new(isolate_scope, "httpGet").unwrap();
        global.set(http_get_key.into(), http_get_fn.into());

        let http_post_fn =
            v8::FunctionTemplate::new(isolate_scope, functions::requests::http::http_post);
        let http_post_key = v8::String::new(isolate_scope, "httpPost").unwrap();
        global.set(http_post_key.into(), http_post_fn.into());
        // ---

        // Modules Reading
        /*let import_fn = v8::FunctionTemplate::new(isolate_scope, import_module_callback);
        let key = v8::String::new(isolate_scope, "import").unwrap();
        global.set(key.into(), import_fn.into());
*/
        let context = v8::Context::new_from_template(isolate_scope, global);
        let context_scope = v8::ContextScope::new(isolate_scope, context);

        Runtime {
            context_scope,
            module_cache: HashMap::new(),
        }
    }
    // Not replit.com
    pub fn repl(&mut self) {
        println!(
            "Yesnt Runtime REPL (V8, {}) - Platform: {:?}",
            v8::V8::get_version(),
            v8::V8::get_current_platform()
        );
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut buff = String::new();

            match io::stdin().read_line(&mut buff) {
                Ok(n) => {
                    if n == 0 {
                        println!();
                        return;
                    }
                    if let Some(result) = self.run(&buff, "(shell)") {}
                }
                Err(error) => println!("error: {}", error),
            }
        }
    }
    fn run(&mut self, script: &str, filename: &str) -> Option<String> {
        let scope = &mut v8::HandleScope::new(&mut self.context_scope);
        let mut try_catch = v8::TryCatch::new(scope);

        let filename = v8::String::new(&mut try_catch, filename).unwrap();
        let undefined = v8::undefined(&mut try_catch);
        let script = v8::String::new(&mut try_catch, script).unwrap();
        let origin = v8::ScriptOrigin::new(
            &mut try_catch,
            filename.into(),
            0,
            0,
            false,
            0,
            Some(undefined.into()),
            false,
            false,
            false,
            None,
        );

        let script = match v8::Script::compile(&mut try_catch, script, Some(&origin)) {
            Some(s) => s,
            None => {
                assert!(try_catch.has_caught());
                let exception = try_catch.exception().unwrap();
                let exception_string = exception.to_string(&mut try_catch).unwrap();
                eprintln!(
                    "An error occurred when compiling the JavaScript: {}",
                    exception_string.to_rust_string_lossy(&mut try_catch)
                );
                return None;
            }
        };

        match script.run(&mut try_catch) {
            Some(result) => Some(
                result
                    .to_string(&mut try_catch)
                    .unwrap()
                    .to_rust_string_lossy(&mut try_catch),
            ),
            None => {
                assert!(try_catch.has_caught());
                let exception = try_catch.exception().unwrap();
                let exception_string = exception.to_string(&mut try_catch).unwrap();
                eprintln!(
                    "An error occurred when running the JavaScript: {}",
                    exception_string.to_rust_string_lossy(&mut try_catch)
                );
                None
            }
        }
    }
    fn import_module<'a>(
        &mut self,
        scope: &'a mut v8::HandleScope<'a>,
        module_name: &str,
    ) -> Option<v8::Local<'a, v8::Value>> {
        // Check if the module is already in the cache
        if let Some(module) = self.module_cache.get(module_name) {
            // Convert the stored global handle back to a local handle
            return Some(v8::Local::new(scope, module));
        }
    
        // Load the module script using a custom module loader (presumably)
        let module = Module::load(module_name).ok()?;
        
        // Run the script and get the result
        let result = self.run(&module.script, module_name)?;
    
        // Create a new V8 scope to manage local handles
        let scope = &mut v8::HandleScope::new(scope);
    
        // Convert the Rust string to a V8 string if the result is of String type
        let local_value: v8::Local<v8::Value> = match v8::String::new(scope, &result) {
            Some(s) => s.into(),
            None => {
                eprintln!("Failed to create V8 string");
                return None;
            }
        };
    
        // Store the value in a V8 global handle for caching
        let global_result: v8::Global<v8::Value> = v8::Global::new(scope, local_value);
        self.module_cache.insert(module_name.to_string(), global_result);
    
        // Return the local value
        Some(local_value)
    }
}

// Thanks to the tutorial: https://dev.to/otterlord/create-your-own-javascript-runtime-10a4

#[allow(unused_variables)]
fn main() {
    let cmd = Command::new("yesnt")
        .bin_name("yesnt")
        .subcommand_required(false)
        .subcommand(
            Command::new("run")
                .about("File to run")
                .arg(Arg::new("file").help("The file to run").required(true)),
        );
    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            if let Some(file_path) = sub_matches.get_one::<String>("file") {
                // Initialize the V8 runtime and execute the script
                let platform = v8::new_default_platform(0, false).make_shared();
                v8::V8::initialize_platform(platform);
                v8::V8::initialize();

                let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
                let handle_scope = &mut v8::HandleScope::new(isolate);

                let mut runtime = Runtime::new(handle_scope);

                // Load the script from the specified file
                let script =
                    std::fs::read_to_string(file_path).expect("Failed to read the script file");

                if let Some(result) = runtime.run(&script, file_path) {
                } else {
                    eprintln!("Failed to run the script.");
                }
            } else {
                eprintln!("No file specified.");
            }
        }
        _ => {
            // Start the REPL if no subcommand is given
            // Initialize the V8 runtime and start the REPL
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();

            let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
            let handle_scope = &mut v8::HandleScope::new(isolate);

            let mut runtime = Runtime::new(handle_scope);

            runtime.repl();
        }
    }
}
