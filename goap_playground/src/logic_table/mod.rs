use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::fs;

use md_logic::logic_table::{parse as read_table, run_table};

pub async fn get_table_eval() -> Result<impl IntoResponse, (StatusCode, String)> {
    let contents = r#"
    | season   | guestCount | desiredDish |
    |----------|------------|------------:|
    | string   | number     |      string |
    | ##       | ##         |          ## |
    | "Fall"   | 8          | "Spaceribs" |
    | "Winter" | 8          | "Roastbeef" |"#;

    let table = read_table(&contents);

    match table {
        Ok(t) => {
            let json_str = r#"
        { "season": "Fall", "guestCount": 8 }
        "#;

            let context: serde_json::Value = serde_json::from_str(json_str).unwrap();
            let res = run_table(&t, &context);

            match res {
                Ok(output) => Ok(Json(output)),
                Err(e) => Err((StatusCode::BAD_REQUEST, e)),
            }
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}

pub async fn md_logic_data_schema() -> Response {
    let contents = fs::read_to_string("./data/md_logic/table.md")
        .expect("Something went wrong reading md_logic file");

    let table = read_table(&contents);

    match table {
        Ok(t) => {
            let mut schema_main = String::new();
            schema_main.push_str("{");

            let mut schema_parts = vec![];

            for (name, in_type) in t.defs.inputs {
                // schema.push_str("\"  \"");
                let mut var_schema = String::new();
                var_schema.push_str("\"");
                var_schema.push_str(&name);
                var_schema.push_str("\": {");
                var_schema.push_str("\"type\": \"");
                var_schema.push_str(&in_type);
                var_schema.push_str("\"");
                var_schema.push_str("}");

                schema_parts.push(var_schema);
            }

            let parts_str = schema_parts.join(",");
            schema_main.push_str(&parts_str);
            schema_main.push_str("}");
            schema_main.into_response()
        }
        Err(e) => "{}".into_response(),
    }
    // return contents.into_response();
}

pub async fn md_logic_uischema() -> Response {
    return "{}".into_response();
}
pub async fn md_logic_inputs() -> Response {
    return "{}".into_response();
}

// md_logic_data_schema, md_logic_inputs, md_logic_uischema
