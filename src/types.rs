use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Variable {
    name: String,
    is_constant: bool,
    values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct JwtLoginOptions {
    url: String,
    login_payload: serde_json::Map<String, Value>,
    token_key_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct QueryDefinition {
    name: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Endpoint {
    name: String,
    is_enabled: bool,
    base_url: String,
    method: String,
    payload: String,
    authorization: String,
    jwt_login: JwtLoginOptions,
    headers: HashMap<String, String>,
    excluded_validators: Vec<String>,
    query_parameters: Vec<QueryDefinition>,
    format: String,
    variables: Vec<Variable>,
    ok_code: i32,
    response_schema: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct EndpointResponse {
    status_code: i32,
    raw_data: Value,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ValidationItem {
    response: EndpointResponse,
    definition: Endpoint,
}
