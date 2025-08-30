//! Course data, such as department, school, instructors, and previous sections. Sourced from the UCI Course Catalog and WebSoc.
use serde_json;

#[path = "./common.rs"]
mod common;

/// Retrieves courses with the IDs provided
///
/// # Parameters
/// - `parameters`: A list of strings containing UCI course IDs
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let parameters = ["COMPSCI161", "COMPSCI162"];
/// let result = retrieve_a_course_with_ids(&parameters);
/// println!("{}", result["data"][0]["id"]);
/// ```
/// 
/// Result: `COMPSCI161`
pub fn retrieve_a_course_with_ids(parameters: &[&str]) -> serde_json::Value {
    let convert: String = common::convert_arr_to_string(parameters);
    let parameters = [("ids", &convert[..])];

    common::get_with_parameters("/v2/rest/courses/batch", &parameters)
}

/// Retrieves a course by its ID.
///
/// # Parameters
/// - `parameter`: A string containing a course ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = "COMPSCI161";
/// let result = retrieve_a_course(&parameter);
/// println!("{}", result["data"]["id"]);
/// ```
/// 
/// Result: `COMPSCI161`
pub fn retrieve_a_course(id: &str) -> serde_json::Value {
    let id: String = format!("/v2/rest/courses/{id}");
    common::get_no_parameters(&id[..])
}

/// Retrieves courses matching the given filters.
///
/// # Parameters
/// - `parameter`: A slice of key-value string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = [("maxUnits", "4")];
/// let result = courses::filter_courses(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn filter_courses(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/courses", parameters)
}

/// Retrieves courses matching the given filters with cursor-based pagination.
///
/// # Parameters
/// - `parameter`: A slice of key-value string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = [("maxUnits", "4")];
/// let result = courses::filter_courses_with_cursor_pagination(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn filter_courses_with_cursor_pagination(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/coursesCursor", parameters)
}