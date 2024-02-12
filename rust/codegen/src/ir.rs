use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;

use crate::json_schema as js;

lazy_static! {
    pub static ref RESERVED_NAMES: HashSet<String> = HashSet::from([
        "abstract".into(),
        "const".into(),
        "for".into(),
        "type".into(),
        "use".into(),
    ]);
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub types: HashMap<String, TypeDef>,
    // types: Vec<TypeDef>,
}

impl Schema {
    pub fn types(&self) -> Vec<(&String, Vec<&TypeDef>)> {
        let mut values: Vec<&TypeDef> = self.types.values().collect();
        values.sort_by(|a, b| match a.module.cmp(&b.module) {
            x @ std::cmp::Ordering::Less | x @ std::cmp::Ordering::Greater => x,
            std::cmp::Ordering::Equal => a.name.cmp(&b.name),
        });

        let mut modules = Vec::new();
        for (module, types) in values.into_iter().group_by(|t| &t.module).into_iter() {
            modules.push((module, types.collect()));
        }
        modules
    }
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub module: String,
    pub name: String,
    pub description: Option<String>,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub enum Type {
    BuiltIn(BuiltIn),
    Struct(Vec<FieldDef>),
    Union(Vec<UnionType>),
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub description: Option<String>,
    pub r#type: FieldType,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    BuiltIn(BuiltIn),
    Ref(String),
    Set(Vec<String>),
    Array(ArrayType),
}

#[derive(Debug, Clone)]
pub enum UnionType {
    Ref(String),
}

#[derive(Debug, Clone)]
pub enum ArrayType {
    Ref(String),
    Set(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum BuiltIn {
    Bool,
    Float,
    Int,
    String,
    UInt,
}

impl Schema {
    pub fn from_json_schema(jschema: js::Schema) -> Result<Schema> {
        // let mut schema = Schema { types: Default::default() };

        let mut types: HashMap<String, TypeDef> = HashMap::new();
        if let Some(ref discriminator) = jschema.discriminator {
            for (name, _) in &discriminator.mapping {
                collect_typedef(name, &jschema, &mut types)?;
            }

            Ok(Self { types })
        } else {
            Err(anyhow!("Invalid FHIR JSON schema"))
        }
    }
}

fn collect_typedef(
    name: &str,
    jschema: &js::Schema,
    types: &mut HashMap<String, TypeDef>,
) -> Result<()> {
    let jdef = &jschema.definitions[name];
    let r#type = if let Some(opts) = &jdef.one_of {
        let options = parse_union(&opts)?;
        Type::Union(options)
    } else if let Some(props) = &jdef.properties {
        let properties =
            parse_fields(props).with_context(|| format!("parse properties: {}", name))?;
        Type::Struct(properties)
    } else if let Some(r#type) = &jdef.r#type {
        let builtin = parse_builtin(name, r#type)?;
        Type::BuiltIn(builtin)
    } else {
        Type::BuiltIn(BuiltIn::String)
    };
    let (module, name) = parse_full_name(name);
    let description = jdef.description.as_ref().map(|d| sanitize_desc(&d));
    let def = TypeDef { name, description, module, r#type };

    let mut refs = HashSet::new();
    collect_deps(&def.r#type, &mut refs);

    types.insert(def.name.clone(), def);
    for name in refs {
        if types.contains_key(&name) {
            continue;
        }
        collect_typedef(&name, jschema, types)?;
    }

    Ok(())
}

fn parse_union(opts: &Vec<js::ClassReference>) -> Result<Vec<UnionType>> {
    let union = opts
        .iter()
        .map(|opt| UnionType::Ref(sanitize_ref(&opt.r#ref).to_string()))
        .collect();
    Ok(union)
}

fn parse_fields(props: &BTreeMap<String, js::Property>) -> Result<Vec<FieldDef>> {
    let mut fields = props
        .iter()
        .map(|(name, prop)| {
            parse_field(name, prop).with_context(|| format!("parse field: {}", name))
        })
        .collect::<Result<Vec<FieldDef>>>()?;
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(fields)
}

fn parse_field(name: &str, jprop: &js::Property) -> Result<FieldDef> {
    let description = jprop.description.as_ref().map(|d| sanitize_desc(&d));
    let r#type = if let Some(r) = &jprop.r#ref {
        FieldType::Ref(sanitize_ref(r).to_string())
    } else if let Some(items) = &jprop.items {
        let r#type = match items {
            js::Items::Ref(r) => ArrayType::Ref(sanitize_ref(r).to_string()),
            js::Items::Enum(vs) => ArrayType::Set(vs.clone()),
        };
        FieldType::Array(r#type)
    } else if let Some(_) = &jprop.r#const {
        FieldType::BuiltIn(BuiltIn::String)
    } else if let Some(opts) = &jprop.r#enum {
        FieldType::Set(opts.clone())
    } else if let Some(r#type) = &jprop.r#type {
        let builtin = parse_builtin(name, r#type)?;
        FieldType::BuiltIn(builtin)
    } else {
        return Err(anyhow!(""));
    };

    let name = name.to_string();
    Ok(FieldDef { name, description, r#type })
}

fn parse_builtin(name: &str, r#type: &str) -> Result<BuiltIn> {
    match r#type {
        "string" => Ok(BuiltIn::String),
        "boolean" => Ok(BuiltIn::Bool),
        "number" => {
            let name = name.to_ascii_lowercase();
            if name.ends_with("decimal") {
                Ok(BuiltIn::Float)
            } else if name.ends_with("integer") {
                Ok(BuiltIn::Int)
            } else if name.ends_with("positiveint") || name.ends_with("unsignedint") {
                Ok(BuiltIn::UInt)
            } else {
                Err(anyhow!("invalid number: {}", name))
            }
        }
        _ => Err(anyhow!("invalid type: {}", r#type)),
    }
}

fn collect_deps(t: &Type, collector: &mut HashSet<String>) -> () {
    match t {
        Type::BuiltIn(_) => (),
        Type::Struct(vv) => {
            for v in vv {
                match &v.r#type {
                    FieldType::BuiltIn(_) => (),
                    FieldType::Set(_) => (),
                    FieldType::Array(v) => match v {
                        ArrayType::Set(_) => (),
                        ArrayType::Ref(r) => {
                            collector.insert(r.to_owned());
                        }
                    },
                    FieldType::Ref(r) => {
                        collector.insert(r.to_owned());
                    }
                }
            }
        }
        Type::Union(vv) => {
            for v in vv {
                match v {
                    UnionType::Ref(r) => {
                        collector.insert(r.to_owned());
                    }
                }
            }
        }
    }
}

fn parse_full_name(name: &str) -> (String, String) {
    match name.split_once("_") {
        Some((m, _)) => (m.to_string(), name.to_string()),
        None => (name.to_string(), name.to_string()),
    }
}

fn sanitize_desc(d: &str) -> String {
    d.replace("\r", "\n")
}

fn sanitize_ref(r: &str) -> &str {
    &r[("#/definitions/".len())..]
}
