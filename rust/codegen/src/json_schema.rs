use std::collections::HashMap;

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
    pub definitions: HashMap<String, Definition>,
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
    pub mapping: HashMap<String, String>,
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
#[serde(rename_all = "camelCase", untagged, deny_unknown_fields)]
pub enum Property {
    Reference {
        description: Option<String>,
        #[serde(rename = "$ref")]
        r#ref: String,
    },
    Array {
        description: Option<String>,
        items: Items,
        r#type: String,
    },
    PatternedTyped {
        description: Option<String>,
        pattern: String,
        r#type: String,
    },
    Typed {
        description: Option<String>,
        r#type: String,
    },
    Enum {
        description: Option<String>,
        r#enum: Vec<String>,
    },
    Const {
        description: Option<String>,
        r#const: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Definition {
    pub pattern: Option<String>,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, Property>>,
    pub additional_properties: Option<bool>,
    pub required: Option<Vec<String>>,
    pub one_of: Option<Vec<HashMap<String, String>>>,
}
