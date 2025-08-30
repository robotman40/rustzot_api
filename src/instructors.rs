use serde_json;

#[path = "./common.rs"]
mod common;

pub fn retrieve_instructors_with_ucinetids(parameters: &[&str]) -> serde_json::Value {
    let convert: String = common::convert_arr_to_string(parameters);
    let parameters = [("ucinetids", &convert[..])];

    common::get_with_parameters("/v2/rest/instructors/batch", &parameters)
}

pub fn retrieve_an_instructor(ucinetid: &str) -> serde_json::Value {
    let id: String = format!("/v2/rest/instructors/{ucinetid}");
    common::get_no_parameters(&id[..])
}

pub fn filter_instructors(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/instructors", parameters)
}

pub fn filter_instructors_with_cursor_pagination(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/instructorsCursor", parameters)
}

