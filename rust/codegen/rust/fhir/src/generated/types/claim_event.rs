use super::*;
/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug,Clone,PartialEq)]
pub struct ClaimEvent {
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
/// A coded event such as when a service is expected or a card printed.
pub r#type: CodeableConcept,
/// A date or period in the past or future indicating when the event occurred or is
/// expectd to occur.
pub when_date_time: String,
/// A date or period in the past or future indicating when the event occurred or is
/// expectd to occur.
pub when_period: Period,
}