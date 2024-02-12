use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Schema {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub id: String,
    pub description: String,
    pub discriminator: Option<Discriminator>,
    pub one_of: Option<Vec<ClassReference>>,
    pub definitions: BTreeMap<String, Definition>,
}

impl Schema {
    pub fn from_reader(rdr: impl std::io::Read) -> serde_json::Result<Schema> {
        serde_json::from_reader(rdr)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Discriminator {
    pub property_name: String,
    pub mapping: BTreeMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClassReference {
    #[serde(rename = "$ref")]
    pub r#ref: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum Items {
    #[serde(rename = "$ref")]
    Ref(String),
    Enum(Vec<String>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Property {
    pub description: Option<String>,
    pub r#type: Option<String>,
    #[serde(rename = "$ref")]
    pub r#ref: Option<String>,
    pub r#enum: Option<Vec<String>>,
    pub r#const: Option<String>,
    pub items: Option<Items>,
    pub pattern: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Definition {
    pub pattern: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub properties: Option<BTreeMap<String, Property>>,
    pub additional_properties: Option<bool>,
    pub required: Option<Vec<String>>,
    pub one_of: Option<Vec<ClassReference>>,
}
