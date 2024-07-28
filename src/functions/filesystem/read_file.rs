use std::fs;
use v8;

#[allow(dead_code)]
pub fn read_file(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    let file_path = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
    match fs::read_to_string(file_path) {
        Ok(content) => rv.set(v8::String::new(scope, &content).unwrap().into()),
        Err(err) => rv.set(v8::String::new(scope, &format!("Error reading file: {}", err)).unwrap().into()),
    }
}