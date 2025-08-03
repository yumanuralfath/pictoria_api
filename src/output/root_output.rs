use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ApiInfo<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub description: &'a str,
    pub status: &'a str,
    pub timestamp: String,
    pub documentation: &'a str,
    pub links: HashMap<&'a str, &'a str>,
}
