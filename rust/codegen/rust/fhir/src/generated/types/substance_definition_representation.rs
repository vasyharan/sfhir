use super::*;
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstanceDefinitionRepresentation {
/// An attached file with the structural representation e.g. a molecular structure
/// graphic of the substance, a JCAMP or AnIML file.
pub document: Reference,
/// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB,
/// mmCIF. The logical content type rather than the physical file format of a
/// document.
pub format: CodeableConcept,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// May be used to represent additional information that is not part of the basic
/// definition of the element and that modifies the understanding of the element
/// in which it is contained and/or the understanding of the containing element's
/// descendants. Usually modifier elements provide negation or qualification.
/// To make the use of extensions safe and managable, there is a strict set
/// of governance applied to the definition and use of extensions. Though any
/// implementer can define an extension, there is a set of requirements that SHALL
/// be met as part of the definition of the extension. Applications processing a
/// resource are required to check for modifier extensions.
///
/// Modifier extensions SHALL NOT change the meaning of any elements on Resource
/// or DomainResource (including cannot change the meaning of modifierExtension
/// itself).
pub modifier_extension: Vec<Extension>,
/// The structural representation as a text string in a standard format.
pub representation: String,
/// The kind of structural representation (e.g. full, partial).
pub r#type: CodeableConcept,
}
