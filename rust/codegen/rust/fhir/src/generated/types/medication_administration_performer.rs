use super::*;
/// Describes the event of a patient consuming or otherwise being administered
/// a medication.  This may be as simple as swallowing a tablet or it may be a
/// long running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationAdministrationPerformer {
/// Indicates who or what performed the medication administration.
pub actor: CodeableReference,
/// Distinguishes the type of involvement of the performer in the medication
/// administration.
pub function: CodeableConcept,
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
}
