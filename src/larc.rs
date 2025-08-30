//! Present and past LARC (https://larc.uci.edu/) sections. Sourced from LARC's enrollment site (https://enroll.larc.uci.edu/).
use serde_json;

#[path = "./common.rs"]
mod common;

/// Retrieves LARC sections data matching the given filters.
///
/// # Parameters
/// - `parameter`: A slice of key-value string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = [("instructorName", "Peter Anteater")];
/// let result = larc::query_larc_sections(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn query_larc_sections(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/larc", parameters)
}