use serde_json;

#[path = "./common.rs"]
mod common;

pub fn query_larc_sections(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/larc", parameters)
}