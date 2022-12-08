extern crate serde_json;
use serde_json::Number;
use serde_json::Value as JsonValue;

use crate::expression_parser::operand::Operand;

pub fn get_context_var(name: &String, context: &serde_json::Value) -> serde_json::Value {
    let v: Vec<&str> = name.split('.').collect();

    let mut cur: &serde_json::Value = &context;
    for key in v.iter() {
        cur = match &cur[key] {
            JsonValue::Null => return JsonValue::Null,
            val => &val,
        }
    }

    cur.clone()
}

pub fn var_to_operand(name: &String, context: &serde_json::Value) -> Operand {
    let v = get_context_var(name, &context);
    match v {
        JsonValue::String(s) => Operand::Primitive(JsonValue::String(s)), //Operand::String(s),
        JsonValue::Number(n) => {
            let n_value = n.as_f64().unwrap_or(0.0);

            Operand::Primitive(JsonValue::Number(Number::from_f64(n_value).unwrap()))
        }
        _ => Operand::Primitive(JsonValue::Null),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn succeeds_get_context_value_as_operand() -> Result<(), String> {
        let json_str = r#"{ "season": "Fall", "preferences": { "type": "vegetarian" }, "count": 1 }
                "#;

        let context: serde_json::Value = serde_json::from_str(json_str).unwrap();

        let c = var_to_operand(&"season".to_owned(), &context);
        assert_eq!(c, Operand::Primitive(JsonValue::String("Fall".to_owned())));

        let num = var_to_operand(&"count".to_owned(), &context);
        assert_eq!(
            num,
            Operand::Primitive(JsonValue::Number(Number::from_f64(1.0).unwrap()))
        );

        let missing_val = var_to_operand(&"NoExist.subParam".to_owned(), &context);
        assert_eq!(missing_val, Operand::Primitive(JsonValue::Null));

        Ok(())
    }
    #[test]
    fn succeeds_get_context_value() -> Result<(), String> {
        let json_str = r#"{ "season": "Fall", "preferences": { "type": "vegetarian" }, "count": 1 }
                "#;

        let context: serde_json::Value = serde_json::from_str(json_str).unwrap();

        let season = get_context_var(&"season".to_string(), &context);

        assert_eq!(season, JsonValue::String("Fall".to_string()));

        let p_type = get_context_var(&"preferences.type".to_string(), &context);
        assert_eq!(p_type, JsonValue::String("vegetarian".to_string()));

        let missing_val = get_context_var(&"NoExist".to_string(), &context);
        assert_eq!(missing_val, JsonValue::Null);

        let missing_val = get_context_var(&"NoExist.subParam".to_string(), &context);
        assert_eq!(missing_val, JsonValue::Null);

        let num = get_context_var(&"count".to_string(), &context);
        let num_value: Option<f64> = match num {
            JsonValue::Number(column_number) => column_number.as_f64(),
            _ => Some(0.0),
        };

        assert_eq!(num_value, Some(1.0));

        Ok(())
    }
}
