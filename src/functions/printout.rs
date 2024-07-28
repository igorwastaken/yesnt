use v8;

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn printout(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    for i in 0..args.length() {
        let arg = args.get(i);
        let message = if arg.is_string() {
            arg.to_string(scope).unwrap().to_rust_string_lossy(scope)
        } else if arg.is_function() {
            let func = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            format!("[Function: {}]", func)
        } else if arg.is_array() {
            let array = v8::Local::<v8::Array>::try_from(arg).unwrap();
            let length = array.length();
            let mut elements = Vec::new();
            for i in 0..length {
                let element = array.get_index(scope, i).unwrap();
                elements.push(element.to_string(scope).unwrap().to_rust_string_lossy(scope));
            }
            format!("[{}]", elements.join(", "))
        } else if arg.is_object() {
            let obj = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            format!("{{Object: {}}}", obj)
        } else if arg.is_number() {
            arg.to_number(scope).unwrap().value().to_string()
        } else {
            arg.to_string(scope).unwrap().to_rust_string_lossy(scope)
        };
        println!("{}", message);
    }
}
