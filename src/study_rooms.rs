use serde_json;

#[path = "./common.rs"]
mod common;

pub fn retrieve_a_study_room(id: &str) -> serde_json::Value {
    let id: String = String::from(format!("/v2/rest/studyRooms/{id}"));
    common::get_no_parameters(&id)
}

pub fn retrieve_study_rooms(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/studyRooms", &parameters)
}