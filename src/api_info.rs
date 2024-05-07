use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: bool,
    api_level: u32,
    api_compatible: u32,
    api_prerelease: bool,
    build: String,
}

mod option {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        println!("start");
        let s: Option<u32> = Option::deserialize(deserializer)?;
        println!("end");
        return Ok(s);
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Function {
    name: String,
    method: bool,
    parameters: Vec<[String; 2]>,
    since: u32,
    #[serde(deserialize_with = "option::deserialize", default)]
    deprecated_since: Option<u32>,
    return_type: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ApiInfo {
    version: Version,
    functions: Vec<Function>,
}
