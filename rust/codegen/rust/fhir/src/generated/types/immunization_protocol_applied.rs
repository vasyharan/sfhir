use super::*;
/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug,Clone,PartialEq)]
pub struct ImmunizationProtocolApplied {
/// Indicates the authority who published the protocol (e.g. ACIP) that is being
/// followed.
pub authority: Reference,
/// Nominal position in a series as intended by the practitioner administering the
/// dose.
pub dose_number: String,
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
/// One possible path to achieve presumed immunity against a disease - within the
/// context of an authority.
pub series: String,
/// The recommended number of doses to achieve immunity as intended by the
/// practitioner administering the dose.
pub series_doses: String,
/// The vaccine preventable disease the dose is being administered against.
pub target_disease: Vec<CodeableConcept>,
}