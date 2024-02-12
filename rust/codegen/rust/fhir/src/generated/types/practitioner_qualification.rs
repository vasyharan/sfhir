use super::*;
/// A person who is directly or indirectly involved in the provisioning of
/// healthcare or related services.
#[derive(Debug,Clone,PartialEq)]
pub struct PractitionerQualification {
/// Coded representation of the qualification.
pub code: CodeableConcept,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// An identifier that applies to this person's qualification.
pub identifier: Vec<Identifier>,
/// Organization that regulates and issues the qualification.
pub issuer: Reference,
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
/// Period during which the qualification is valid.
pub period: Period,
}
