//! Data concerning AP Exams as they relate to UCI.
use serde_json;

#[path = "./common.rs"]
mod common;

/// Get a mapping from AP exam names as they appear in the UCI Catalogue to their official names as given by College Board
///
/// # Parameters
/// - `parameters`: A slice of key-value string pairs representing query parameters.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let parameters = [
///     ("fullName", "AP Microeconomics"),
///     ("catalogueName", "AP ECONOMICS:MICRO")
/// ];
/// let result = ap::retrieve_ap_exam_names(&parameters);
/// println!("{}", result["data"][0]["fullName"]);
/// ```
/// 
/// Result: `AP Microeconomics`
pub fn retrieve_ap_exam_names(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/apExams", parameters)
}