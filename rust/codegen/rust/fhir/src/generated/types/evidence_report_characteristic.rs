use super::*;
/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts.
#[derive(Debug,Clone,PartialEq)]
pub struct EvidenceReportCharacteristic {
/// Characteristic code.
pub code: CodeableConcept,
/// Is used to express not the characteristic.
pub exclude: Boolean,
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
/// Timeframe for the characteristic.
pub period: Period,
/// Characteristic value.
pub value_boolean: bool,
/// Characteristic value.
pub value_codeable_concept: CodeableConcept,
/// Characteristic value.
pub value_quantity: Quantity,
/// Characteristic value.
pub value_range: Range,
/// Characteristic value.
pub value_reference: Reference,
}