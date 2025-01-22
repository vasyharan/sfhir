use std::fs::File;
use std::path::PathBuf;
use std::process;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use convert_case::{Case, Casing};
use zip::read::ZipArchive;

mod schema;

#[derive(Debug, Parser)]
pub struct Args {
    /// Paths to files to open
    crate_path: Option<PathBuf>,
}

enum FhirRelease {
    R5,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let resources_dir = PathBuf::from("./resources");
    ensure_dir(&resources_dir)?;

    let crate_path = args.crate_path.unwrap_or(PathBuf::from("rust/fhir"));
    let generated_dir = crate_path.join("src").join("generated");
    std::fs::remove_dir_all(&generated_dir)?;
    ensure_dir(&generated_dir)?;

    let resources = get_fhir_resources(FhirRelease::R5, &resources_dir)?;
    let mut generated_mod = File::create(generated_dir.join("mod.rs"))?;
    for structure_definition in resources.structures_definitions {
        use std::io::Write;

        if structure_definition.kind != schema::StructureDefinitionKind::Resource
            || structure_definition.derivation != Some(schema::TypeDerivationRule::Specialization)
            || structure_definition.r#abstract
        {
            continue;
        }

        let modname = structure_definition.get_structure_field_name();
        let file = File::create(generated_dir.join(format!("{}.rs", modname)))?;
        if let Some(el) = structure_definition.snapshot.element.get(0) {
            generate_structs(&file, &structure_definition, el)?;
        }

        writeln!(generated_mod, "mod {};", modname)?;
        writeln!(generated_mod, "pub use {}::*;", modname)?;
        writeln!(generated_mod)?;
    }
    rust_fmt()?;

    // let

    // let file = File::open(filepath).with_context(|| format!("path {}", filepath.as_os_str))?;
    // let json_schema = json_schema::Schema::from_reader(file)
    //     .with_context(|| format!("read JSON schema {}", filepath))?;
    // let schema = ir::Schema::from_json_schema(json_schema)?;

    // let resources_dir = generated_dir.join("resources");
    // ensure_dir(&resources_dir)?;

    // let mut resources_mod = File::create(resources_dir.join("mod.rs"))?;
    // for name in schema.modules {
    //     let s = &schema.types[&name];
    //     let mut resource = File::create(resources_dir.join(format!("{}.rs", s.mod_name())))?;
    //     if let Some(ref desc) = s.description {
    //         writeln!(resource, "{}", textwrap::indent(&textwrap::fill(desc, 80), "/// "))?;
    //     }
    //     writeln!(resource, "#[derive(Debug,Clone,PartialEq)]")?;
    //     writeln!(resource, "pub struct {} {{", s.type_name())?;
    //     for prop in &s.properties {
    //         writeln!(resource, "pub {}: {},", prop.name, prop.r#type)?;
    //     }
    //     writeln!(resource, "}}")?;

    //     writeln!(resources_mod, "mod {};", s.mod_name())?;
    //     writeln!(resources_mod, "pub use {}::*;", s.mod_name())?;
    //     writeln!(resources_mod)?;
    // }

    // let mut generated_mod = File::create(generated_dir.join("mod.rs"))?;
    // writeln!(generated_mod, "mod resources;")?;

    // rust_fmt()?;

    Ok(())
}

fn generate_structs(
    mut w: impl std::io::Write,
    def: &schema::StructureDefinition,
    el: &schema::ElementDefinition,
) -> Result<()> {
    let typename = def.get_container_type_name(el);
    let children = def.get_direct_children(el);
    writeln!(w, "#[derive(Debug,Clone,PartialEq)]")?;
    writeln!(w, "pub struct {} {{", typename)?;
    for child in children {
        // if let Some(ref short) = child.short {
        //     writeln!(w, "{}", textwrap::indent(&textwrap::fill(short, 80), "/// "))?;
        // }
        if let Some(ref definition) = child.definition {
            writeln!(w, "{}", textwrap::indent(&textwrap::fill(definition, 80), "/// "))?;
        }
        writeln!(
            w,
            "pub {}: {},",
            def.get_element_field_name(child),
            def.get_element_type_name(child),
        )?;
    }
    writeln!(w, "}}")?;
    Ok(())
}

fn get_fhir_resources(release: FhirRelease, resources_dir: &PathBuf) -> Result<schema::Schema> {
    use std::io::Write;

    let (filepath, url) = match release {
        FhirRelease::R5 => ("definitions-r5.json.zip", "http://hl7.org/fhir/r5/definitions.json.zip"),
    };

    let filepath = resources_dir.join(filepath);
    if !filepath.exists() {
        let resp = reqwest::blocking::Client::builder()
            .build()?
            .get(url)
            .header("Accept", "application/x-zip-compressed")
            .header("User-Agent", "sfhir/dev")
            .send()?;
        let mut file = File::create(&filepath)?;
        file.write_all(&resp.bytes()?)?;
    }

    let file = File::open(&filepath).with_context(|| format!("path {:?}", filepath))?;
    let zip = ZipArchive::new(file)?;
    let resources = schema::from_zip(zip)?;
    Ok(resources)
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
