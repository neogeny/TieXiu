use serde_json::{json, Value, Map};

pub fn cst_to_json(cst: &Cst) -> Value {
    match cst {
        // Simple terminals
        Cst::Token(s) | Cst::Literal(s) => Value::String(s.to_string()),

        // Passthrough for single items
        Cst::Item(child) => cst_to_json(child),

        // Arrays
        Cst::List(items) | Cst::Closure(items) => {
            let array = items.iter().map(|c| cst_to_json(c)).collect();
            Value::Array(array)
        }

        // Object with a single key mapping to a value
        Cst::Named(name, child) => {
            let mut obj = Map::new();
            obj.insert(name.to_string(), cst_to_json(child));
            Value::Object(obj)
        }

        // Object with a single key mapping to an array of one value
        Cst::NamedList(name, child) => {
            let mut obj = Map::new();
            obj.insert(name.to_string(), Value::Array(vec![cst_to_json(child)]));
            Value::Object(obj)
        }

        // The constructed Ast: convert the internal HashMap to a JSON Object
        Cst::Ast(ast) => {
            let mut obj = Map::new();
            for (key, value) in &ast.fields {
                obj.insert(key.clone(), cst_to_json(value));
            }
            Value::Object(obj)
        }

        // Null/Empty
        Cst::Nil => Value::Null,
    }
}
