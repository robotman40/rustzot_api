//! Historical enrollment data for UCI. Sourced from WebSoc.
use serde_json;

#[path = "./common.rs"]
mod common;

/// Retrieves historical enrollment data for the given parameters. Granular history arrays only available for recent terms.
///
/// # Parameters
/// - `parameter`: A slice of key-value string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = [("year", "2024")];
/// let result = instructors::filter_enrollment_history(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn filter_enrollment_history(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/enrollmentHistory", parameters)
}