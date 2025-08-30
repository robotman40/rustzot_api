use reqwest::{blocking, Url};
use serde_json;

pub(crate) fn get_no_parameters(get: &str) -> serde_json::Value {
    let result = blocking::get(format!("https://anteaterapi.com{}", get)).unwrap()
    .text()
    .unwrap();

    serde_json::from_str(&result).unwrap()
}

pub(crate) fn get_with_parameters(get: &str, parameters: &[(&str, &str)]) -> serde_json::Value {
    let url = String::from(format!("https://anteaterapi.com{}", get));

    let fetch = Url::parse_with_params(&url, parameters).unwrap();

    let result = blocking::get(fetch).unwrap()
        .text()
        .unwrap();

    serde_json::from_str(&result).unwrap()
}

pub(crate) fn convert_arr_to_string(arr: &[&str]) -> String {
    let mut new_str = String::new();

    for (i, item) in arr.iter().enumerate() {
        new_str.push_str(item);
        if i != arr.len() - 1 {
            new_str.push_str(",");
        }
    }

    new_str
}