use super::*;
/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug,Clone,PartialEq)]
pub struct VerificationResultAttestation {
/// The method by which attested information was submitted/retrieved (manual; API;
/// Push).
pub communication_method: CodeableConcept,
/// The date the information was attested to.
pub date: Date,
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
/// When the who is asserting on behalf of another (organization or individual).
pub on_behalf_of: Reference,
/// A digital identity certificate associated with the proxy entity submitting
/// attested information on behalf of the attestation source.
pub proxy_identity_certificate: String,
/// Signed assertion by the proxy entity indicating that they have the right to
/// submit attested information on behalf of the attestation source.
pub proxy_signature: Signature,
/// A digital identity certificate associated with the attestation source.
pub source_identity_certificate: String,
/// Signed assertion by the attestation source that they have attested to the
/// information.
pub source_signature: Signature,
/// The individual or organization attesting to information.
pub who: Reference,
}
