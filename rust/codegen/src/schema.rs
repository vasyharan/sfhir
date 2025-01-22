use std::collections::{HashMap, HashSet};
use std::io::BufReader;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use serde::{de::Visitor, Deserialize};
use zip::ZipArchive;

lazy_static! {
    #[rustfmt::skip]
    static ref STRUCTURE_DEFINITION_RENAMES: HashMap<&'static str, &'static str> = [
        ("http://hl7.org/fhir/StructureDefinition/valueset-reference", "ValueSetReference"),
        ("http://hl7.org/fhir/StructureDefinition/codesystem-reference", "CodeSystemReference"),
        ("http://hl7.org/fhir/StructureDefinition/request-statusReason", "StatusReasonExtension"),
        ("http://hl7.org/fhir/StructureDefinition/workflow-relatedArtifact", "RelatedArtifactExtension"),
        ("http://hl7.org/fhir/StructureDefinition/event-location", "LocationExtension"),
        ("http://hl7.org/fhir/StructureDefinition/workflow-episodeOfCare", "EpisodeOfCareExtension"),
        ("http://hl7.org/fhir/StructureDefinition/workflow-researchStudy", "ResearchStudyExtension"),
        ("http://hl7.org/fhir/us/core/StructureDefinition/us-core-condition", "UsCoreCondition"),
        ("http://hl7.org/fhir/us/core/StructureDefinition/us-core-direct", "UsCoreDirectEmail"),
        ("http://hl7.org/fhir/StructureDefinition/hdlcholesterol", "HdlCholesterol"),
        ("http://hl7.org/fhir/StructureDefinition/ldlcholesterol", "LdlCholesterol"),
        ("http://hl7.org/fhir/StructureDefinition/lipidprofile", "LipidProfile"),
        ("http://hl7.org/fhir/StructureDefinition/cholesterol", "Cholesterol"),
        ("http://hl7.org/fhir/StructureDefinition/triglyceride", "Triglyceride"),
        ("http://hl7.org/fhir/StructureDefinition/cqf-expression", "CqfExpression"),
        ("http://hl7.org/fhir/StructureDefinition/cqf-library", "CqfLibrary"),
    ]
    .iter()
    .copied()
    .collect();
}

const BUNDLE_STRUCTURE_DEFINITION_URL: &str = "http://hl7.org/fhir/StructureDefinition/Bundle";
const EXPLICIT_TYPE_NAME_EXTENSION_URL: &str =
    "http://hl7.org/fhir/StructureDefinition/structuredefinition-explicit-type-name";

#[derive(Debug)]
pub struct Schema {
    pub structures_definitions: Vec<StructureDefinition>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinition {
    pub id: String,
    pub url: String,
    pub kind: StructureDefinitionKind,
    pub derivation: Option<TypeDerivationRule>,
    pub r#abstract: bool,
    pub snapshot: Snapshot,
    pub extension: Option<Vec<Extension>>,
}

impl StructureDefinition {
    pub fn get_structure_type_name(&self) -> String {
        self.get_structure_name().to_case(Case::Pascal)
    }

    pub fn get_structure_field_name(&self) -> String {
        self.get_structure_name().to_case(Case::Snake)
    }

    fn get_structure_name(&self) -> &str {
        STRUCTURE_DEFINITION_RENAMES
            .get(self.url.as_str())
            .map(|s| *s)
            .unwrap_or_else(|| {
                if self.derivation == Some(TypeDerivationRule::Constraint) {
                    panic!("constraint: {}", self.id)
                } else {
                    self.id.as_str()
                }
            })
    }

    pub fn get_element_type_name(&self, el: &ElementDefinition) -> String {
        if !el.is_choice_type()
            && el
                .r#type
                .get(0)
                // .as_ref()
                // .and_then(|t| t.get(0))
                .map(|t| t.code.as_str())
                .unwrap_or_default()
                == "Reference"
        {
            return self.get_container_type_name(el);
        }

        if let Some(r#type) = el.r#type.get(0) {
            match r#type.code.as_str() {
                "http://hl7.org/fhirpath/System.String" | "string" => return "String".to_string(),
                _ => {}
            }
        }

        // if el.is_choice_type() || el.is_container() {
        //     self.get_container_type_name(el)
        // } else {
        //     assert!(el.r#type.len() == 1);
        //     let code = &el.r#type[0].code;
        //     if code == "http://hl7.org/fhirpath/System.String" {
        //         return "String".to_string();
        //     }
        //     self.get_container_type_name(el)
        //     // unimplemented!()
        // }
        self.get_container_type_name(el)
    }

    pub fn get_container_type_name(&self, el: &ElementDefinition) -> String {
        match el.content_reference {
            Some(ref content_reference) => {
                let referenced_id = content_reference[1..].to_string();
                let referenced_el = self
                    .get_element_by_id(&referenced_id)
                    .expect("referenced element not found");
                if !referenced_el.is_container() {
                    unimplemented!()
                }
                self.get_element_type_name(referenced_el)
            }
            None => {
                let explicit_type_name: Option<&ExtensionValue> = el.extension.as_ref().and_then(|extensions| {
                    extensions
                        .iter()
                        .find(|e| {
                            e.url == EXPLICIT_TYPE_NAME_EXTENSION_URL && matches!(e.value, ExtensionValue::String(_))
                        })
                        .map(|e| &e.value)
                });
                let type_name = if let Some(v) = explicit_type_name {
                    match v {
                        ExtensionValue::String(s) => s.clone(),
                        _ => unreachable!(),
                    }
                } else {
                    el.get_element_name(Case::Pascal)
                };
                type_name
            }
        }
    }

    pub fn get_element_field_name(&self, el: &ElementDefinition) -> String {
        el.get_element_name(Case::Snake)
    }

    pub fn get_direct_children<'a>(&'a self, el: &ElementDefinition) -> Vec<&'a ElementDefinition> {
        let path_parts_len = el.id.split('.').count();
        let descendants = self.get_descendants(el);
        descendants
            .iter()
            .filter(|e| e.id.split('.').count() == path_parts_len + 1)
            .copied()
            .collect()
    }

    pub fn get_descendants<'a>(&'a self, el: &ElementDefinition) -> Vec<&'a ElementDefinition> {
        let prefix = format!("{}.", el.id);
        self.snapshot
            .element
            .iter()
            .filter(|e| e.id.starts_with(&prefix))
            .collect()
    }

    fn get_element_by_id<'a>(&'a self, id: &str) -> Option<&'a ElementDefinition> {
        self.snapshot.element.iter().find(|e| e.id == id)
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum StructureDefinitionKind {
    PrimitiveType,
    ComplexType,
    Resource,
    Logical,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub element: Vec<ElementDefinition>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition {
    pub id: String,
    pub content_reference: Option<String>,
    pub extension: Option<Vec<Extension>>,
    #[serde(default)]
    pub r#type: Vec<ElementType>,
    pub short: Option<String>,
    pub definition: Option<String>,
}

#[derive(Debug)]
struct IdToken {}

impl IdToken {
    fn from_str(s: &str) -> Self {
        Self {}
    }
}

impl ElementDefinition {
    // pub fn get_element_field_name(&self) -> String {
    //     self.get_element_name(Case::Snake)
    // }

    fn get_element_name(&self, casing: Case) -> String {
        let name = if let Some(part) = self.id.split('.').last() {
            part
        } else {
            &self.id
        };
        let name = name.to_case(casing);
        let name = if name.ends_with("[x]") {
            name[..(name.len() - 3)].to_string()
        } else {
            name
        };

        match name.as_str() {
            "type" | "use" | "for" | "abstract" | "const" => format!("r#{}", name),
            _ => name,
        }
    }

    fn is_container(&self) -> bool {
        // if self.r#type.as_ref().map(|t| t.len()).unwrap_or(0) != 1 {
        if self.r#type.len() != 1 {
            return false;
        }
        if self.id.find(".").is_none() {
            return true;
        }
        let r#type = &self.r#type[0].code;
        return r#type == "BackboneElement" || r#type == "Element";
    }

    fn is_choice_type(&self) -> bool {
        let id = self.id.split(".").last().unwrap();
        id.ends_with("[x]")
    }

    fn get_distinct_type_count(&self) -> usize {
        let len = self.r#type.len();
        if len < 2 {
            return len;
        }
        self.r#type.iter().collect::<HashSet<_>>().len()
    }

    // fn is_container(&self) -> bool {
    //     if self.r#type.as_ref().map(|t| t.len()).unwrap_or(0) != 1 {
    //         return false;
    //     }
    //     if self.id.find(".").is_none() {
    //         return true;
    //     }
    //     let r#type = &self.r#type.as_ref().unwrap()[0].code;
    //     return r#type == "BackboneElement" || r#type == "Element";
    // }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ElementType {
    pub code: String,
    pub targetProfile: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Extension {
    pub url: String,
    pub value: ExtensionValue,
}

impl<'de> Deserialize<'de> for Extension {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        enum Field {
            Url,
            ValueString,
            ValueInteger,
            ValueCode,
            ValueUri,
            ValueBoolean,
            ValueMarkdown,
        }

        struct ExtensionVisitor;
        impl<'de> Visitor<'de> for ExtensionVisitor {
            type Value = Extension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Extension")
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut url = None;
                let mut value = None;

                while let Some(key) = map.next_key()? {
                    if key == Field::Url {
                        if url.is_some() {
                            return Err(serde::de::Error::duplicate_field("url"));
                        }
                        url = Some(map.next_value()?);
                    } else {
                        if value.is_some() {
                            return Err(serde::de::Error::duplicate_field("valueString"));
                        }

                        match key {
                            Field::Url => unreachable!(),
                            Field::ValueString => value = Some(ExtensionValue::String(map.next_value()?)),
                            Field::ValueInteger => value = Some(ExtensionValue::Integer(map.next_value()?)),
                            Field::ValueCode => value = Some(ExtensionValue::Code(map.next_value()?)),
                            Field::ValueUri => value = Some(ExtensionValue::Uri(map.next_value()?)),
                            Field::ValueBoolean => value = Some(ExtensionValue::Boolean(map.next_value()?)),
                            Field::ValueMarkdown => value = Some(ExtensionValue::Markdown(map.next_value()?)),
                        }
                    }
                }
                let url = url.ok_or_else(|| serde::de::Error::missing_field("url"))?;
                let value = value.ok_or_else(|| serde::de::Error::missing_field("value[x]"))?;
                Ok(Extension { url, value })
            }
        }

        deserializer.deserialize_struct("Extension", &[], ExtensionVisitor)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionValue {
    String(String),
    Integer(i64),
    Code(String),
    Uri(String),
    Boolean(bool),
    Markdown(String),
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum TypeDerivationRule {
    Specialization,
    Constraint,
}

enum ResourceType {
    Ignore,
    Bundle,
    StructureDefinition,
}

pub fn from_zip(mut zip: ZipArchive<std::fs::File>) -> Result<Schema> {
    let mut resources = Schema {
        structures_definitions: Vec::new(),
    };

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        if file.is_dir() {
            continue;
        }

        let filename = file.name().to_string();
        if filename.ends_with(".json") {
            let file = BufReader::new(file);
            let json_value: serde_json::Value =
                serde_json::from_reader(file).with_context(|| format!("read json file {}", filename))?;
            collect_structure_definitions(&mut resources, json_value)?;
        }
    }

    Ok(resources)
}

fn collect_structure_definitions(resources: &mut Schema, json_value: serde_json::Value) -> Result<()> {
    match get_resource_type(&json_value)? {
        ResourceType::Ignore => {}
        ResourceType::StructureDefinition => {
            resources
                .structures_definitions
                .push(serde_json::from_value(json_value)?);
        }
        ResourceType::Bundle => {
            let entries = get_bundle_entries(json_value)?;
            for entry in entries {
                let entry = get_entry(entry)?;
                collect_structure_definitions(resources, entry)?;
            }
        }
    }
    Ok(())
}

fn get_resource_type(value: &serde_json::Value) -> Result<ResourceType> {
    match value {
        serde_json::Value::Object(map) => match map.get("resourceType") {
            Some(resource_type) => match resource_type {
                serde_json::Value::String(resource_type) => match resource_type.as_str() {
                    "StructureDefinition" => Ok(ResourceType::StructureDefinition),
                    "Bundle" => Ok(ResourceType::Bundle),
                    _ => Ok(ResourceType::Ignore),
                },
                _ => Err(anyhow!("resourceType is not a string")),
            },
            None => Err(anyhow!("no resourceType field")),
        },
        _ => Err(anyhow!("not an object")),
    }
}

fn get_bundle_entries(value: serde_json::Value) -> Result<Vec<serde_json::Value>> {
    match value {
        serde_json::Value::Object(mut map) => match map.remove("entry") {
            Some(entry) => match entry {
                serde_json::Value::Array(entries) => Ok(entries),
                _ => Err(anyhow!("entry is not an array")),
            },
            None => Err(anyhow!("no entry field")),
        },
        _ => Err(anyhow!("not an object")),
    }
}

fn get_entry(value: serde_json::Value) -> Result<serde_json::Value> {
    match value {
        serde_json::Value::Object(map) => match map.get("resource") {
            Some(resource) => Ok(resource.clone()),
            None => Err(anyhow!("no resource field")),
        },
        _ => Err(anyhow!("not an object")),
    }
}
