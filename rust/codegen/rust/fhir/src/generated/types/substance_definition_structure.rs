use super::*;
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstanceDefinitionStructure {
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
/// An expression which states the number and type of atoms present in a molecule of
/// a substance.
pub molecular_formula: String,
/// Specified per moiety according to the Hill system, i.e. first C, then H, then
/// alphabetical, each moiety separated by a dot.
pub molecular_formula_by_moiety: String,
/// The molecular weight or weight range (for proteins, polymers or nucleic acids).
pub molecular_weight: SubstanceDefinitionMolecularWeight,
/// Optical activity type.
pub optical_activity: CodeableConcept,
/// A depiction of the structure of the substance.
pub representation: Vec<SubstanceDefinitionRepresentation>,
/// The source of information about the structure.
pub source_document: Vec<Reference>,
/// Stereochemistry type.
pub stereochemistry: CodeableConcept,
/// The method used to elucidate the structure of the drug substance. Examples: X-
/// ray, NMR, Peptide mapping, Ligand binding assay.
pub technique: Vec<CodeableConcept>,
}
