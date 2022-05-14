#![no_main]
use libfuzzer_sys::fuzz_target;

use valico::json_schema;
use serde_json::Value;
use std::string::String;

fuzz_target!(|data: &[u8]| {
    let mut scope = json_schema::Scope::new();
    match String::from_utf8(data.to_vec()) {
        Ok(s) => {
            let _ = scope.compile(Value::String(s), true);
        },
        _ => {},
    }
});
