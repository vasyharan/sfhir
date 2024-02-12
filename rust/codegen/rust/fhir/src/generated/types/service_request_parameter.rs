use super::*;
/// A record of a request for service such as diagnostic investigations, treatments,
/// or operations to be performed.
#[derive(Debug,Clone,PartialEq)]
pub struct ServiceRequestParameter {
/// A value representing the additional detail or instructions for the order (e.g.,
/// catheter insertion, body elevation, descriptive device configuration and/or
/// setting instructions).
pub code: CodeableConcept,
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
/// Indicates a value for the order detail.
pub value_boolean: bool,
/// Indicates a value for the order detail.
pub value_codeable_concept: CodeableConcept,
/// Indicates a value for the order detail.
pub value_period: Period,
/// Indicates a value for the order detail.
pub value_quantity: Quantity,
/// Indicates a value for the order detail.
pub value_range: Range,
/// Indicates a value for the order detail.
pub value_ratio: Ratio,
/// Indicates a value for the order detail.
pub value_string: String,
}
