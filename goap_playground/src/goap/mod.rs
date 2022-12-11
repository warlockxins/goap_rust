use axum::response::{IntoResponse, Response};
use std::fs;
use tera::{Context, Tera};

mod planner;
pub use planner::goap_run;

pub async fn goap_data_schema() -> Response {
    let contents = fs::read_to_string("./schemas/goap/schema.jsontpl")
        .expect("Something went wrong reading the file");

    let parameters = fs::read_to_string("./data/goap/parameters.json")
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

pub async fn goap_action_data() -> Response {
    let contents = fs::read_to_string("./data/goap/actions.json")
        .expect("Something went wrong reading actions file");

    return contents.into_response();
}

pub async fn goap_uischema() -> Response {
    let contents = fs::read_to_string("./schemas/goap/uischema.json")
        .expect("Something went wrong reading the file");

    return contents.into_response();
}
