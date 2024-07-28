// src/functions/http.rs

use ureq;
use v8;

pub fn http_get(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    let url = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
    match ureq::get(&url).call() {
        Ok(response) => {
            let body = response.into_string().unwrap_or_else(|_| "Failed to read response body".to_string());
            rv.set(v8::String::new(scope, &body).unwrap().into());
        }
        Err(err) => rv.set(v8::String::new(scope, &format!("HTTP GET request failed: {:?}", err)).unwrap().into()),
    }
}

pub fn http_post(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    let url = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
    let body = args.get(1).to_string(scope).unwrap().to_rust_string_lossy(scope);
    let response = ureq::post(&url).send_string(&body);
    match response {
        Ok(response) => {
            let body = response.into_string().unwrap_or_else(|_| "Failed to read response body".to_string());
            rv.set(v8::String::new(scope, &body).unwrap().into());
        }
        Err(err) => rv.set(v8::String::new(scope, &format!("HTTP POST request failed: {:?}", err)).unwrap().into()),
    }
}