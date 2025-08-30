use serde_json;

#[path = "./common.rs"]
mod common;

pub fn ping() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/ping")
}

pub fn retrieve_search_results(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/search", parameters)
}