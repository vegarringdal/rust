use std::ptr::null;

use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    if john["wow"].is_null() {
        println!("first phone number: {}", john["wow"][0]);
    }

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
