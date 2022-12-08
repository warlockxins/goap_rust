use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract, Json};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use tera::{Context, Tera};

use std::fs;

mod planner;
use planner::goap_run;

use md_logic::logic_table::{parse as read_table, run_table};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/", get(root))
        .route("/uischema", get(goap_uischema))
        .route("/dataschema", get(goap_data_schema))
        .route("/data/actions", get(goap_action_data))
        .route("/table_logic", get(get_table_eval))
        .route("/run", post(goap_run))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn goap_action_data() -> Response {
    let contents = fs::read_to_string("./data/actions.json")
        .expect("Something went wrong reading actions file");

    return contents.into_response();
}

async fn goap_uischema() -> Response {
    let contents = fs::read_to_string("./schemas/uischema.json")
        .expect("Something went wrong reading the file");

    return contents.into_response();
}

async fn get_table_eval() -> Result<impl IntoResponse, (StatusCode, String)> {
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

async fn goap_data_schema() -> Response {
    let contents = fs::read_to_string("./schemas/schema.jsontpl")
        .expect("Something went wrong reading the file");

    let parameters = fs::read_to_string("./data/parameters.json")
        .expect("Something went wrong reading the  parameters file");

    let mut tera = Tera::default();

    let template_namespace = "schema";
    tera.add_raw_template(template_namespace, &contents)
        .unwrap();

    let mut context = Context::new();
    context.insert("parameters", &parameters);

    let config = tera.render(template_namespace, &context).unwrap();

    return config.into_response();
}
