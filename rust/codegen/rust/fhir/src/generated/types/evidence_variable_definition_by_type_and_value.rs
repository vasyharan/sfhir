use super::*;
/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug,Clone,PartialEq)]
pub struct EvidenceVariableDefinitionByTypeAndValue {
/// Device used for determining characteristic.
pub device: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Method for how the characteristic value was determined.
pub method: Vec<CodeableConcept>,
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
/// Defines the reference point for comparison when valueQuantity or valueRange is
/// not compared to zero.
pub offset: CodeableConcept,
/// Used to express the type of characteristic.
pub r#type: CodeableConcept,
/// Defines the characteristic when paired with characteristic.type.
pub value_boolean: bool,
/// Defines the characteristic when paired with characteristic.type.
pub value_codeable_concept: CodeableConcept,
/// Defines the characteristic when paired with characteristic.type.
pub value_id: String,
/// Defines the characteristic when paired with characteristic.type.
pub value_quantity: Quantity,
/// Defines the characteristic when paired with characteristic.type.
pub value_range: Range,
/// Defines the characteristic when paired with characteristic.type.
pub value_reference: Reference,
}
