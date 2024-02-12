use super::*;
/// Specifies contact information for a specific purpose over a period of time,
/// might be handled/monitored by a specific named person or organization.
#[derive(Debug,Clone,PartialEq)]
pub struct ExtendedContactDetail {
/// Address for the contact.
pub address: Address,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The name of an individual to contact, some types of contact detail are usually
/// blank.
pub name: Vec<HumanName>,
/// This contact detail is handled/monitored by a specific organization. If the name
/// is provided in the contact, then it is referring to the named individual within
/// this organization.
pub organization: Reference,
/// Period that this contact was valid for usage.
pub period: Period,
/// The purpose/type of contact.
pub purpose: CodeableConcept,
/// The contact details application for the purpose defined.
pub telecom: Vec<ContactPoint>,
}
