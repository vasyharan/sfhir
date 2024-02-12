use super::*;
/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
/// Encounter is primarily used to record information about the actual activities
/// that occurred, where Appointment is used to record planned activities.
#[derive(Debug,Clone,PartialEq)]
pub struct EncounterAdmission {
/// From where patient was admitted (physician referral, transfer).
pub admit_source: CodeableConcept,
/// Location/organization to which the patient is discharged.
pub destination: Reference,
/// Category or kind of location after discharge.
pub discharge_disposition: CodeableConcept,
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
/// The location/organization from which the patient came before admission.
pub origin: Reference,
/// Pre-admission identifier.
pub pre_admission_identifier: Identifier,
/// Indicates that this encounter is directly related to a prior admission,
/// often because the conditions addressed in the prior admission were not fully
/// addressed.
pub re_admission: CodeableConcept,
}
