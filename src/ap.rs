use serde_json;

#[path = "./common.rs"]
mod common;

pub fn retrieve_ap_exam_names(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/apExams", parameters)
}