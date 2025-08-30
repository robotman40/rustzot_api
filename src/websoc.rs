use serde_json;

#[path = "./common.rs"]
mod common;

pub fn query_websoc(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/websoc", parameters)
}

pub fn list_available_websoc_terms() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/websoc/terms")
}

