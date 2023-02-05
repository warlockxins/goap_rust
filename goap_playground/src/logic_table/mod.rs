use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::fs;
use tera::{Context, Tera};

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
            let contents = fs::read_to_string("./schemas/md_logic/data_schema.jsontpl")
                .expect("Something went wrong reading md_logic data schema file");

            let mut tera = Tera::default();

            let template_namespace = "md table data schema";
            tera.add_raw_template(template_namespace, &contents)
                .unwrap();

            let mut context = Context::new();
            context.insert("parameters", &t.defs.inputs);

            let ui_config = tera.render(template_namespace, &context).unwrap();

            return ui_config.into_response();
        }
        Err(e) => "{}".into_response(),
    }
    // return contents.into_response();
}

pub async fn md_logic_uischema() -> Response {
    let table_contents = fs::read_to_string("./data/md_logic/table.md")
        .expect("Something went wrong reading md_logic file for uischema");

    let table = read_table(&table_contents);

    match table {
        Ok(t) => {
            let contents = fs::read_to_string("./schemas/md_logic/uischema.jsontpl")
                .expect("Something went wrong reading md_logic schema file");

            let names: Vec<String> = t.defs.inputs.into_iter().map(|(name, _)| name).collect();

            let mut tera = Tera::default();

            let template_namespace = "md table ui schema";
            tera.add_raw_template(template_namespace, &contents)
                .unwrap();

            let mut context = Context::new();
            context.insert("parameters", &names);

            let ui_config = tera.render(template_namespace, &context).unwrap();

            return ui_config.into_response();
        }
        Err(e) => "{}".into_response(),
    }
}
pub async fn md_logic_inputs() -> Response {
    return "{}".into_response();
}

// md_logic_data_schema, md_logic_inputs, md_logic_uischema
