//! Information about study rooms at UCI
use serde_json;

#[path = "./common.rs"]
mod common;

/// Retrieves a study room by its ID.
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("44670");
/// let result = study_rooms::retrieve_a_study_room(&parameter);
/// println!("{}", result["data"]["id"]);
/// ```
/// 
/// Result: `44670`
pub fn retrieve_a_study_room(id: &str) -> serde_json::Value {
    let id: String = String::from(format!("/v2/rest/studyRooms/{id}"));
    common::get_no_parameters(&id)
}

/// Retrieves study rooms matching the given filters. If no filters are provided, all rooms are returned.
///
/// # Parameters
/// - `parameters`: A slice of key-value string pairs representing query parameters.
///    The parameter "quarter" is required to work.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let parameters = [
///     ("location", "Science Library"),
/// ];
/// let result = study_rooms::retrieve_study_rooms(&parameters);
/// println!("{}", result["data"][0]["id"]);
/// ```
/// 
/// Result: `2024`
pub fn retrieve_study_rooms(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/studyRooms", &parameters)
}