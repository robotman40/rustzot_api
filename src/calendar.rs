//! Core calendar dates and current week.
use serde_json;

#[path = "./common.rs"]
mod common;

/// Retrieves key dates for the provided term.
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
///     ("quarter", "Fall")
/// ];
/// let result = calendar::retrieve_term_calendar(&parameters);
/// println!("{}", result["data"]["year"]);
/// ```
/// 
/// Result: `2024`
pub fn retrieve_term_calendar(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/calendar", parameters)
}

/// Retrieves all data for all terms that are currently available.
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let result = calendar::retrieve_all_calendars();
/// println!("{}", result["data"]["year"]);
/// ```
/// 
/// Result: It will depend on the time
pub fn list_all_calendars() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/calendar/all")
}

/// Retrieves key dates for the provided term.
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
///     ("month", "9")
///     ("day", "30")
/// ];
/// let result = calendar::retrieve_current_week(&parameters);
/// println!("{}", result["data"]["weeks"][0]);
/// ```
/// 
/// Result: `1`
pub fn retrieve_current_week(parameters: &[(&str, &str)]) -> serde_json::Value {
    common::get_with_parameters("/v2/rest/week", parameters)
}