//! Historical grade data for UCI classes, sourced via California Public Records Act (CPRA) requests. Plus / minus data not available. Data for sections with less than 10 students not available.
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
/// let result = grades::filter_grades(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn filter_grades(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/raw", parameters)
}

/// Retrieves raw grades data for the given parameters.
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
/// let result = grades::filter_grade_options(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn filter_grade_options(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/options", parameters)
}

/// Retrieves grades aggregated by section and the set of sections that are included in this aggregation.
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
/// let result = grades::retrieve_grade_aggregate(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn retrieve_grade_aggregate(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/aggregate", parameters)
}

/// Retrieves grades aggregated by course and the set of courses that are included in this aggregation.
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
/// let result = grades::retrieve_grade_aggregate_by_course(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn retrieve_grade_aggregate_by_course(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/aggregateByCourse", parameters)
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
/// let parameters = [("year", "2024")];
/// let result = grades::retrieve_grade_aggregate_by_offering(&parameters);
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn retrieve_grade_aggregate_by_offering(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/grades/aggregateByOffering", parameters)
}