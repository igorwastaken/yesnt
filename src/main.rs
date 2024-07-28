mod functions;
use clap::{Arg, Command};
use std::io::{self, Write};

struct Runtime<'s, 'i> {
    context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
}
#[allow(unused_variables)]
impl<'s, 'i> Runtime<'s, 'i>
where
    's: 'i,
{
    pub fn new(isolate_scope: &'i mut v8::HandleScope<'s, ()>) -> Self {
        let global = v8::ObjectTemplate::new(isolate_scope);

        // printout function
        let printout_fn = v8::FunctionTemplate::new(isolate_scope, functions::printout::printout);
        let key = v8::String::new(isolate_scope, "printout").unwrap();
        global.set(key.into(), printout_fn.into());
        // ---
        let context = v8::Context::new_from_template(isolate_scope, global);
        let context_scope = v8::ContextScope::new(isolate_scope, context);

        Runtime { context_scope }
    }
    // Not replit.com
    #[allow(dead_code)]
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
                    if let Some(result) = self.run(&buff, "(shell)") {
                        println!("{}", result);
                    }
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
            Some(result) => Some(result.to_string(&mut try_catch).unwrap().to_rust_string_lossy(&mut try_catch)),
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
            eprintln!("Invalid subcommand.");
        }
    }
}
