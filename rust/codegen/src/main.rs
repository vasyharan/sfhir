use anyhow::{anyhow, Context, Result};
use clap::Parser;
use convert_case::{Case, Casing};
use std::fs::File;
use std::path::PathBuf;
use std::process;

mod ir;
mod json_schema;

#[derive(Debug, Parser)]
pub struct Args {
    /// Paths to files to open
    crate_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    use std::io::Write;

    // TODO: consider using FHIR spec (rather than the JSON schema)
    // refer to: https://github.com/google/fhir/blob/master/java/com/google/fhir/protogen/ProtoGeneratorV2.java

    let args = Args::parse();
    let filepath = "/workspace/sfhir/resources/fhir.schema.json";
    let file = File::open(filepath).with_context(|| format!("path {}", filepath))?;
    let json_schema = json_schema::Schema::from_reader(file)
        .with_context(|| format!("read JSON schema {}", filepath))?;
    let schema = ir::Schema::from_json_schema(json_schema)?;

    let crate_path = args.crate_path.unwrap_or(PathBuf::from("rust/fhir"));
    let generated_dir = crate_path.join("src").join("generated");
    if generated_dir.exists() {
        std::fs::remove_dir_all(&generated_dir).context("remove generated dir")?;
    }
    ensure_dir(&generated_dir)?;

    let mut generated_mod = File::create(generated_dir.join("mod.rs"))?;
    writeln!(generated_mod, "#![allow(unused_imports)]")?;
    writeln!(generated_mod)?;

    for (name, types) in schema.types() {
        // let mod_name = rust_modname(&name);
        let mod_name = name.to_case(Case::Snake);
        // let type_name = rust_typename(&name);
        let type_name = name.to_case(Case::UpperCamel);
        writeln!(generated_mod, "mod {};", mod_name)?;
        writeln!(generated_mod, "pub use {}::{};", mod_name, type_name)?;
        writeln!(generated_mod)?;

        // let mut w = File::create(types_dir.join(format!("{}.rs", mod_name)))?;
        // generate_type(&mut w, &s)?;

        let mut w = File::create(generated_dir.join(format!("{}.rs", mod_name)))?;
        // writeln!(w, "use super::*;")?;
        for t in &types {
            generate_type(&mut w, &schema, &t)?;
            writeln!(w)?;
        }
    }

    // writeln!(generated_mod, "mod resources;")?;
    // let resources_dir = generated_dir.join("resources");
    // ensure_dir(&resources_dir)?;
    // let mut resources_mod = File::create(resources_dir.join("mod.rs"))?;
    // for (name, module) in schema.resources() {
    //     if name != "Account" {
    //         continue;
    //     }
    //     // let s = &module.structs[name];
    //     let mod_name = rust_modname(&name);
    //     let type_name = rust_typename(&name);
    //     writeln!(resources_mod, "mod {};", mod_name)?;
    //     writeln!(resources_mod, "pub use {}::{};", mod_name, type_name)?;
    //     writeln!(resources_mod)?;

    //     let mut w = File::create(resources_dir.join(format!("{}.rs", mod_name)))?;
    //     writeln!(w, "use super::super::types::*;")?;
    //     for s in &module {
    //         generate_type(&mut w, &s)?;
    //         writeln!(w)?;
    //     }
    // }

    rust_fmt(&crate_path)?;

    Ok(())
}

fn generate_type(w: &mut File, s: &ir::Schema, t: &ir::TypeDef) -> Result<()> {
    use std::io::Write;

    // let type_name = rust_typename(t);
    let type_name = &t.name.to_case(Case::UpperCamel);
    if let Some(desc) = &t.description {
        writeln!(w, "{}", textwrap::indent(&textwrap::fill(desc, 80), "/// "))?;
    }

    writeln!(w, "#[derive(Debug,Clone,PartialEq)]")?;
    match &t.r#type {
        ir::Type::BuiltIn(b) => {
            // let name = rust_varname(&t.name);
            let name = &t.name.to_case(Case::UpperCamel);
            let type_name = rust_builtin(b);
            writeln!(w, "pub struct {}({});", name, type_name)?;
        }
        ir::Type::Struct(fields) => {
            writeln!(w, "pub struct {} {{", type_name)?;
            for prop in fields {
                let name = rust_varname(&prop.name);
                if prop.name.starts_with("_") || name == "extension" {
                    continue;
                }
                if let Some(desc) = &prop.description {
                    writeln!(w, "{}", textwrap::indent(&textwrap::fill(desc, 80), "/// "))?;
                }
                match &prop.r#type {
                    ir::FieldType::Ref(r) => {
                        let def = &s.types[r];
                        let type_name = rust_typename(def);
                        // let type_name = if &s.name == r {
                        //     format!("Box<{}>", type_name)
                        // } else {
                        //     type_name
                        // };
                        writeln!(w, "pub {}: super::{},", name, type_name)?;
                    }
                    ir::FieldType::Array(a) => match a {
                        ir::ArrayType::Ref(r) => {
                            let def = &s.types[r];
                            let type_name = rust_typename(def);
                            writeln!(w, "pub {}: Vec<super::{}>,", name, type_name)?;
                        }
                        ir::ArrayType::Set(_) => {
                            // let type_name = rust_typename(&prop.name);
                            let type_name = &prop.name.to_case(Case::UpperCamel);
                            writeln!(w, "pub {}: Vec<{}>,", name, type_name)?;
                        }
                    },
                    ir::FieldType::BuiltIn(b) => {
                        let type_name = rust_builtin(b);
                        writeln!(w, "pub {}: {},", name, type_name)?;
                    }
                    ir::FieldType::Set(_) => {
                        // let type_name = rust_typename(&prop.name);
                        let type_name = &prop.name.to_case(Case::UpperCamel);
                        writeln!(w, "pub {}: {},", name, type_name)?;
                    }
                }
            }
            writeln!(w, "}}")?;

            for prop in fields {
                if let ir::FieldType::Set(opts) = &prop.r#type {
                    // let type_name = rust_typename(&prop.name);
                    let type_name = &prop.name.to_case(Case::UpperCamel);
                    writeln!(w)?;
                    writeln!(w, "#[derive(Debug,Clone,PartialEq)]")?;
                    writeln!(w, "pub enum {} {{", type_name)?;
                    for opt in opts {
                        writeln!(w, "{},", rust_enum_name(opt))?;
                    }
                    writeln!(w, "}}")?;
                }
            }
        }
        ir::Type::Union(union) => {
            writeln!(w, "pub enum {} {{", type_name)?;
            for u in union {
                match u {
                    ir::UnionType::Ref(r) => {
                        let def = &s.types[r];
                        let enum_name = def.name.to_case(Case::UpperCamel);
                        let type_name = rust_typename(def);
                        writeln!(w, "{}(super::{}),", enum_name, type_name)?;
                    }
                }
            }
            writeln!(w, "}}")?;
        }
    }

    Ok(())
}

fn rust_builtin(b: &ir::BuiltIn) -> &str {
    match b {
        ir::BuiltIn::Bool => "bool",
        ir::BuiltIn::Float => "f64",
        ir::BuiltIn::Int => "i64",
        ir::BuiltIn::String => "String",
        ir::BuiltIn::UInt => "u64",
    }
}

fn ensure_dir(dir: &PathBuf) -> Result<()> {
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn rust_varname(name: &str) -> String {
    let name = name.to_case(Case::Snake);
    if ir::RESERVED_NAMES.contains(&name) {
        format!("r#{}", name)
    } else {
        name
    }
}

fn rust_enum_name(name: &str) -> String {
    match name {
        "<" => "LT".to_owned(),
        "<=" => "LTE".to_owned(),
        ">" => "GT".to_owned(),
        ">=" => "GTE".to_owned(),
        _ => name.to_case(Case::UpperCamel),
    }
}

fn rust_modname(t: &ir::TypeDef) -> String {
    t.module.to_case(Case::Snake)
}

fn rust_typename(t: &ir::TypeDef) -> String {
    format!("{}::{}", rust_modname(t), t.name.to_case(Case::UpperCamel))
}

fn rust_fmt(dir: &PathBuf) -> Result<()> {
    let status = process::Command::new("cargo")
        .current_dir(dir)
        .args(["fmt"])
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("cargo fmt failed!"))
    }
}
