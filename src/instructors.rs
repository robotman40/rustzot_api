//! Instructor data enriched with course data.
use serde_json;

#[path = "./common.rs"]
mod common;

/// Retrieves instructors with the UCINetIDs provided.
/// - `parameter`: A list of strings containing ucinetid strings
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = ["mikes", "klefstad"];
/// let result = instructors::retrieve_instructors_with_ucinetids(&parameters);
/// println!("{}", result["data"][0]["ucinetid"]);
/// ```
/// 
/// Result: `mikes`
pub fn retrieve_instructors_with_ucinetids(parameters: &[&str]) -> serde_json::Value {
    let convert: String = common::convert_arr_to_string(parameters);
    let parameters = [("ucinetids", &convert[..])];

    common::get_with_parameters("/v2/rest/instructors/batch", &parameters)
}

/// Retrieves an instructor by their UCInetID.
/// - `parameter`: A string containing a ucinetid
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("mikes");
/// let result = instructors::retrieve_an_instructor(&parameter);
/// println!("{}", result["data"]["ucinetid"]);
/// ```
/// 
/// Result: `mikes`
pub fn retrieve_an_instructor(ucinetid: &str) -> serde_json::Value {
    let id: String = format!("/v2/rest/instructors/{ucinetid}");
    common::get_no_parameters(&id[..])
}

/// Retrieves grades aggregated by offering, which is a course and the instructor who taught it, and the set of courses that are included in this aggregation.
///
/// # Parameters
/// - `parameter`: A slice of key-value string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = [("nameContains", "mikes")];
/// let result = instructors::filter_instructors(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn filter_instructors(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/instructors", parameters)
}

/// Retrieves grades aggregated by offering, which is a course and the instructor who taught it, and the set of courses that are included in this aggregation.
///
/// # Parameters
/// - `parameter`: A slice of key-vother::pingalue string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = [("nameContains", "mikes")];
/// let result = instructors::filter_instructors_with_cursor_pagination(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn filter_instructors_with_cursor_pagination(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/instructorsCursor", parameters)
}

