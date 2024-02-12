use super::*;
/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug,Clone,PartialEq)]
pub struct SubstanceDefinitionMoiety {
/// Quantitative value for this moiety.
pub amount_quantity: Quantity,
/// Quantitative value for this moiety.
pub amount_string: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Identifier by which this moiety substance is known.
pub identifier: Identifier,
/// The measurement type of the quantitative value. In capturing the actual relative
/// amounts of substances or molecular fragments it may be necessary to indicate
/// whether the amount refers to, for example, a mole ratio or weight ratio.
pub measurement_type: CodeableConcept,
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
/// Molecular formula for this moiety of this substance, typically using the Hill
/// system.
pub molecular_formula: String,
/// Textual name for this moiety substance.
pub name: String,
/// Optical activity type.
pub optical_activity: CodeableConcept,
/// Role that the moiety is playing.
pub role: CodeableConcept,
/// Stereochemistry type.
pub stereochemistry: CodeableConcept,
}
