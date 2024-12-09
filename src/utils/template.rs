use inflector::Inflector;
use std::collections::HashMap;
use tera::Value;

pub fn snake_case_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let input = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("`snake_case` filter can only be applied to strings"))?;
    Ok(Value::String(input.to_snake_case()))
}

pub fn pascal_case_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let input = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("`pascal_case` filter can only be applied to strings"))?;
    Ok(Value::String(input.to_pascal_case()))
}

pub fn plural_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let input = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("`plural` filter can only be applied to strings"))?;

    let plural = if input.ends_with("s") {
        format!("{}es", input)
    } else {
        format!("{}s", input)
    };

    Ok(Value::String(plural))
}

pub fn snake_case_to_kebab_case_filter(
    value: &Value,
    _: &HashMap<String, Value>,
) -> tera::Result<Value> {
    let input = value.as_str().ok_or_else(|| {
        tera::Error::msg("`snake_case_to_kebab_case` filter can only be applied to strings")
    })?;
    Ok(Value::String(input.to_kebab_case().replace("_", "-")))
}
