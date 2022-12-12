use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod goap;
use goap::{goap_action_data, goap_data_schema, goap_run, goap_uischema};

mod logic_table;
use logic_table::{get_table_eval, md_logic_data_schema, md_logic_inputs, md_logic_uischema};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/", get(root))
        .route("/goap/uischema", get(goap_uischema))
        .route("/goap/dataschema", get(goap_data_schema))
        .route("/data/goap/actions", get(goap_action_data))
        .route("/table_logic", get(get_table_eval))
        .route("/goap/run", post(goap_run))
        // table
        .route("/md_logic/uischema", get(md_logic_uischema))
        .route("/md_logic/dataschema", get(md_logic_data_schema))
        .route("/data/md_logic/actions", get(md_logic_inputs))
        // .route("/md_logic/run", post(goap_run))
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
