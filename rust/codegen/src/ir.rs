use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};

use crate::json_schema::{self, Property as JProperty, Schema as JSchema};

pub struct Schema {
    pub modules: Vec<String>,
    pub types: HashMap<String, Structure>,
}

impl Schema {
    pub fn from_json_schema(jschema: JSchema) -> Result<Schema> {
        let mut schema = Schema { modules: vec![], types: HashMap::new() };

        if let Some(ref discriminator) = jschema.discriminator {
            // for () in discriminator.mapping {}
            let name = "Account".to_string();
            schema.parse_module(name, &jschema)?;
            Ok(schema)
        } else {
            Err(anyhow!("Invalid FHIR JSON schema"))
        }
    }

    fn parse_module(&mut self, name: String, jschema: &JSchema) -> Result<()> {
        let def = &jschema.definitions[&name];
        let s = Self::parse_structure(name.clone(), jschema)
            .with_context(|| format!("invalid defintion: {}", &name))?;
        self.modules.push(s.name.clone());
        self.types.insert(s.name.clone(), s);
        Ok(())
    }

    fn parse_structure(name: String, jschema: &JSchema) -> Result<Structure> {
        let def = &jschema.definitions[&name];
        if let Some(ref opts) = def.one_of {
            todo!()
        } else if let Some(ref props) = def.properties {
            let properties = Self::parse_properties(props)?;
            Ok(Structure { name, description: def.description.clone(), properties })
        } else {
            Err(anyhow!("must contain either options or properties"))
        }
    }

    fn parse_properties(props: &HashMap<String, JProperty>) -> Result<Vec<Property>> {
        let mut properties = vec![];
        for (name, prop) in props {
            if let Some(p) = Self::parse_property(name.into(), prop)? {
                properties.push(p);
            }
        }
        Ok(properties)
    }

    fn parse_property(name: String, prop: &JProperty) -> Result<Option<Property>> {
        if name.starts_with("_") {
            return Ok(None);
        }

        use JProperty::*;
        let (r#type, description): (&str, &Option<String>) = match prop {
            Reference { description, r#ref } => {
                let r#type = &r#ref[("#/definitions/".len())..];
                (r#type, description)
            }
            Array { description, items, r#type } => (r#type, description),
            PatternedTyped { description, pattern, r#type } => todo!(),
            Typed { description, r#type } => (r#type, description),
            Enum { description, r#enum } => todo!(),
            Const { description, r#const } => (r#const, description),
        };

        let name = Self::field_name(name);
        let description = description.clone();
        let r#type = r#type.into();
        let p = Property { name, description, r#type };
        Ok(Some(p))
    }

    fn field_name(name: String) -> String {
        if name == "type" {
            return "r#type".into();
        } else {
            return name.to_case(Case::Snake);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub name: String,
    pub description: Option<String>,
    pub properties: Vec<Property>,
}

impl Structure {
    pub fn mod_name(&self) -> String {
        self.name.to_case(Case::Snake)
    }

    pub fn type_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub description: Option<String>,
    pub r#type: String,
}
