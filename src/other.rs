//! Nonstandard APIs in the Anteater API
use serde_json;

#[path = "./common.rs"]
mod common;

/// An endpoint for testing your connectivity to the REST API. This endpoint is never cached, so you can also use it to check your remaining request quota.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let result = other::ping();
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn ping() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/ping")
}

/// Retrieves course/instructor results for the given search query.
///
/// # Parameters
/// - `parameter`: A slice of key-value string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameters = [("query", "Peter Anteater")];
/// let result = larc::query_larc_sections(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn retrieve_search_results(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/search", parameters)
}