use super::*;
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstanceDefinitionName {
/// The use context of this name for example if there is a different name a drug
/// active ingredient as opposed to a food colour additive.
pub domain: Vec<CodeableConcept>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The jurisdiction where this name applies.
pub jurisdiction: Vec<CodeableConcept>,
/// Human language that the name is written in.
pub language: Vec<CodeableConcept>,
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
/// The actual name.
pub name: String,
/// Details of the official nature of this name.
pub official: Vec<SubstanceDefinitionOfficial>,
/// If this is the preferred name for this substance.
pub preferred: Boolean,
/// Supporting literature.
pub source: Vec<Reference>,
/// The status of the name, for example 'current', 'proposed'.
pub status: CodeableConcept,
/// A synonym of this particular name, by which the substance is also known.
pub synonym: Vec<SubstanceDefinitionName>,
/// A translation for this name into another human language.
pub translation: Vec<SubstanceDefinitionName>,
/// Name type, for example 'systematic',  'scientific, 'brand'.
pub r#type: CodeableConcept,
}
