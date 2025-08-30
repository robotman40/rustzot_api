use serde_json;

#[path = "./common.rs"]
mod common;

pub fn retrieve_majors() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/programs/majors")
}

pub fn retrieve_major(id: &str) ->  serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/majors", &parameter)
}

pub fn retrieve_minors() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/programs/minors")
}

pub fn retrieve_minor(id: &str) ->  serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/minors", &parameter)
}

pub fn retrieve_specializations() -> serde_json::Value {
    common::get_no_parameters("/v2/rest/programs/specializations")
}

pub fn retrieve_specialization(id: &str) ->  serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/specializations", &parameter)
}

pub fn retrieve_major_requirements(program_id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &program_id[..])];
    common::get_with_parameters("/v2/rest/programs/specializations", &parameter)
}

pub fn retrieve_minor_requirements(program_id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &program_id[..])];
    common::get_with_parameters("/v2/rest/programs/minor", &parameter)
}

pub fn retrieve_specialization_requirements(program_id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &program_id[..])];
    common::get_with_parameters("/v2/rest/programs/specialization", &parameter)
}

pub fn retrieve_undergraduate_requirements(id: &str) -> serde_json::Value {
    let key: String = String::from("id");
    let parameter = [(&key[..], &id[..])];
    common::get_with_parameters("/v2/rest/programs/ugradRequirements", &parameter)
}