use super::*;
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstanceDefinitionSourceMaterial {
/// The country or countries where the material is harvested.
pub country_of_origin: Vec<CodeableConcept>,
/// The genus of an organism, typically referring to the Latin epithet of the genus
/// element of the plant/animal scientific name.
pub genus: CodeableConcept,
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
/// An anatomical origin of the source material within an organism.
pub part: CodeableConcept,
/// The species of an organism, typically referring to the Latin epithet of the
/// species of the plant/animal.
pub species: CodeableConcept,
/// A classification that provides the origin of the raw material. Example: cat hair
/// would be an Animal source type.
pub r#type: CodeableConcept,
}
