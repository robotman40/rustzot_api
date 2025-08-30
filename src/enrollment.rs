use serde_json;

#[path = "./common.rs"]
mod common;

pub fn filter_enrollment_history(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/enrollmentHistory", parameters)
}