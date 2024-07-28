use v8;
#[allow(unused_variables)]
pub fn printout(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, rv: v8::ReturnValue) {
    let message = args.get(0);
    if message.is_string() {
        let message = message.to_string(scope).unwrap().to_rust_string_lossy(scope);
        println!("{}", message);
    }
}
