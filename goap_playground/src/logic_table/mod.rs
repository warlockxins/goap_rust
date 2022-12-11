use axum::http::StatusCode;
use axum::response::IntoResponse;

use axum::Json;

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
