use serde_json;

#[path = "./common.rs"]
mod common;

pub fn retrieve_a_course_with_ids(parameters: &[&str]) -> serde_json::Value {
    let convert: String = common::convert_arr_to_string(parameters);
    let parameters = [("ids", &convert[..])];

    common::get_with_parameters("/v2/rest/courses/batch", &parameters)
}

pub fn retrieve_a_course(id: &str) -> serde_json::Value {
    let id: String = format!("/v2/rest/courses/{id}");
    common::get_no_parameters(&id[..])
}

pub fn filter_courses(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/courses", parameters)
}


pub fn filter_courses_with_cursor_pagination(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/coursesCursor", parameters)
}