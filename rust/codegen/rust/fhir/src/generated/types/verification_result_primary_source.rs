use super::*;
/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug,Clone,PartialEq)]
pub struct VerificationResultPrimarySource {
/// Ability of the primary source to push updates/alerts (yes; no; undetermined).
pub can_push_updates: CodeableConcept,
/// Method for communicating with the primary source (manual; API; Push).
pub communication_method: Vec<CodeableConcept>,
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
/// Type of alerts/updates the primary source can send (specific requested changes;
/// any changes; as defined by source).
pub push_type_available: Vec<CodeableConcept>,
/// Type of primary source (License Board; Primary Education; Continuing Education;
/// Postal Service; Relationship owner; Registration Authority; legal source;
/// issuing source; authoritative source).
pub r#type: Vec<CodeableConcept>,
/// When the target was validated against the primary source.
pub validation_date: DateTime,
/// Status of the validation of the target against the primary source (successful;
/// failed; unknown).
pub validation_status: CodeableConcept,
/// Reference to the primary source.
pub who: Reference,
}
