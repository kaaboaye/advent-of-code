use std::collections::HashMap;
use crate::JsonValue::{JsonArray, JsonNull, JsonBoolean, JsonObject, JsonInteger, JsonFloat, JsonString};

#[derive(Debug, Clone)]
enum JsonValue {
    JsonNull,
    JsonBoolean(bool),
    JsonString(String),
    JsonInteger(i64),
    JsonFloat(f64),
    JsonArray(Vec<JsonValue>),
    JsonObject(HashMap<String, JsonValue>),
}

fn main() {
    let mut map: HashMap<String, JsonValue> = HashMap::new();
    map.insert("int".to_string(), JsonInteger(123));
    map.insert("float".to_string(), JsonFloat(1.23));
    map.insert("array of strings".to_string(), JsonArray(vec![
        JsonString("sfdfsd".to_string()),
        JsonString("asdasdasd".to_string()),
    ]));

    let val = JsonArray(vec![
        JsonNull,
        JsonBoolean(true),
        JsonBoolean(false),
        JsonObject(map)
    ]);


    println!("{:?}", val);
}
