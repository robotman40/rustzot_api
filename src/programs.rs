//! Information relating to UCI programs
use serde_json;

#[path = "./common.rs"]
mod common;

/// List all available majors in UCI's current catalogue.
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let result = programs::retrieve_majors();
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn retrieve_majors() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/programs/majors")
}

/// Retrieves a major by its ID.
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("BA-163");
/// let result = programs::retrieve_major(&parameter);
/// println!("{}", result["data"][0]["id"]);
/// ```
/// 
/// Result: `BA-014`
pub fn retrieve_major(id: &str) ->  serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/majors", &parameter)
}

/// List all available minors in UCI's current catalogue.
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let result = programs::retrieve_minors();
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn retrieve_minors() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/programs/minors")
}

/// Retrieves a minor by its ID.
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("49A");
/// let result = programs::retrieve_major(&parameter);
/// println!("{}", result["data"][0]["id"]);
/// ```
/// 
/// Result: `25F`
pub fn retrieve_minor(id: &str) ->  serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/minors", &parameter)
}

/// List all available specializations in UCI's current catalogue.
/// # Returns
/// - `serde_json::Value`: The JSON response containing the response
/// 
/// # Example
/// ```rust
/// let result = programs::retrieve_specializations();
/// println!("{}", result["ok"]);
/// ```
/// 
/// Result: `true`
pub fn retrieve_specializations() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/programs/specializations")
}

/// Retrieves a specialization by its ID.
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("BS-201");
/// let result = programs::retrieve_major(&parameter);
/// println!("{}", result["data"][0]["id"]);
/// ```
/// 
/// Result: `BS-201B`
pub fn retrieve_specialization(id: &str) ->  serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/specializations", &parameter)
}

/// Retrieve course requirements for a major in UCI's current catalogue. Note that these are the requirements for the major itself; if this major has specializations, then one is mandatory and its requirements apply as well.
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("BS-201");
/// let result = programs::retrieve_major_requirements(&parameter);
/// println!("{}", result["data"]["id"]);
/// ```
/// 
/// Result: `BS-201B`
pub fn retrieve_major_requirements(program_id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &program_id[..])];
    common::get_with_parameters("/v2/rest/programs/specializations", &parameter)
}

/// Retrieve course requirements for a minor in UCI's current catalogue.
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("459");
/// let result = programs::retrieve_minor_requirements(&parameter);
/// println!("{}", result["data"]["id"]);
/// ```
/// 
/// Result: `459`
pub fn retrieve_minor_requirements(program_id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &program_id[..])];
    common::get_with_parameters("/v2/rest/programs/minor", &parameter)
}

/// Retrieve course requirements for a specialization in UCI's current catalogue.
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("BS-201E");
/// let result = programs::retrieve_specialization_requirements(&parameter);
/// println!("{}", result["data"]["id"]);
/// ```
/// 
/// Result: `BS-201E`
pub fn retrieve_specialization_requirements(program_id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &program_id[..])];
    common::get_with_parameters("/v2/rest/programs/specialization", &parameter)
}

/// Retrieve requirements external to, but required for, for all undergraduate degrees
/// - `parameter`: A string containing an ID
///
/// # Returns
/// - `serde_json::Value`: The JSON response containing courses with the IDs provided
/// 
/// # Example
/// ```rust
/// let parameter = String::from("UC");
/// let result = programs::retrieve_undergraduate_requirements(&parameter);
/// println!("{}", result["data"]["id"]);
/// ```
/// 
/// Result: `string`
pub fn retrieve_undergraduate_requirements(id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/ugradRequirements", &parameter)
}