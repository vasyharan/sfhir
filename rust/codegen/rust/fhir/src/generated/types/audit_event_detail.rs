use super::*;
/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
#[derive(Debug,Clone,PartialEq)]
pub struct AuditEventDetail {
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
/// The type of extra detail provided in the value.
pub r#type: CodeableConcept,
/// The  value of the extra detail.
pub value_base_64_binary: String,
/// The  value of the extra detail.
pub value_boolean: bool,
/// The  value of the extra detail.
pub value_codeable_concept: CodeableConcept,
/// The  value of the extra detail.
pub value_date_time: String,
/// The  value of the extra detail.
pub value_integer: i64,
/// The  value of the extra detail.
pub value_period: Period,
/// The  value of the extra detail.
pub value_quantity: Quantity,
/// The  value of the extra detail.
pub value_range: Range,
/// The  value of the extra detail.
pub value_ratio: Ratio,
/// The  value of the extra detail.
pub value_string: String,
/// The  value of the extra detail.
pub value_time: String,
}
