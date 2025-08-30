use serde_json;

#[path = "./common.rs"]
mod common;

pub fn retrieve_term_calendar(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/calendar", parameters)
}

pub fn list_all_calendars() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/calendar/all")
}

pub fn retrieve_current_week(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/week", parameters)
}