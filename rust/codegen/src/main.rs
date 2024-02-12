use anyhow::{anyhow, Context, Result};
use clap::Parser;
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
    let filepath = "./resources/fhir.schema.json";
    let file = File::open(filepath).with_context(|| format!("path {}", filepath))?;
    let json_schema = json_schema::Schema::from_reader(file)
        .with_context(|| format!("read JSON schema {}", filepath))?;
    let schema = ir::Schema::from_json_schema(json_schema)?;

    let crate_path = args.crate_path.unwrap_or(PathBuf::from("rust/fhir"));
    let generated_dir = crate_path.join("src").join("generated");
    std::fs::remove_dir_all(&generated_dir)?;
    ensure_dir(&generated_dir)?;

    let resources_dir = generated_dir.join("resources");
    ensure_dir(&resources_dir)?;

    let mut resources_mod = File::create(resources_dir.join("mod.rs"))?;
    for name in schema.modules {
        let s = &schema.types[&name];
        let mut resource = File::create(resources_dir.join(format!("{}.rs", s.mod_name())))?;
        if let Some(ref desc) = s.description {
            writeln!(resource, "{}", textwrap::indent(&textwrap::fill(desc, 80), "/// "))?;
        }
        writeln!(resource, "#[derive(Debug,Clone,PartialEq)]")?;
        writeln!(resource, "pub struct {} {{", s.type_name())?;
        for prop in &s.properties {
            writeln!(resource, "pub {}: {},", prop.name, prop.r#type)?;
        }
        writeln!(resource, "}}")?;

        writeln!(resources_mod, "mod {};", s.mod_name())?;
        writeln!(resources_mod, "pub use {}::*;", s.mod_name())?;
        writeln!(resources_mod)?;
    }

    let mut generated_mod = File::create(generated_dir.join("mod.rs"))?;
    writeln!(generated_mod, "mod resources;")?;

    rust_fmt()?;

    Ok(())
}

fn ensure_dir(dir: &PathBuf) -> Result<()> {
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

fn rust_fmt() -> Result<()> {
    let status = process::Command::new("cargo").args(["fmt"]).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("cargo fmt failed!"))
    }
}
