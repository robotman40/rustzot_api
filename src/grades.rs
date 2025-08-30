use serde_json;

#[path = "./common.rs"]
mod common;

pub fn filter_grades(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/raw", parameters)
}

pub fn filter_grade_options(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/options", parameters)
}

pub fn retrieve_grade_aggregate(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/aggregate", parameters)
}

pub fn retrieve_grade_aggregate_by_course(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/aggregateByCourse", parameters)
}

pub fn retrieve_grade_aggregate_by_offering(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/aggregateByOffering", parameters)
}