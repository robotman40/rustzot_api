//! WebSoc related data, such as valid terms and sections. Sourced directly from WebSoc.
use serde_json;

#[path = "./common.rs"]
mod common;

/// Retrieves WebSoc data satisfying the given parameters.
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
///     ("year", "2024"),
/// ];
/// let result = websoc::retrieve_study_rooms(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn query_websoc(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/websoc", parameters)
}

/// Retrieve all terms currently available on WebSoc.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let result = websoc::list_available_websoc_terms();
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn list_available_websoc_terms() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/websoc/terms")
}

