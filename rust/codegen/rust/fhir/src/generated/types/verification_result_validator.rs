use super::*;
/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug,Clone,PartialEq)]
pub struct VerificationResultValidator {
/// Signed assertion by the validator that they have validated the information.
pub attestation_signature: Signature,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A digital identity certificate associated with the validator.
pub identity_certificate: String,
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
/// Reference to the organization validating information.
pub organization: Reference,
}