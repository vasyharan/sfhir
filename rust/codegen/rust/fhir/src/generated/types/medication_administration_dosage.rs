use super::*;
/// Describes the event of a patient consuming or otherwise being administered
/// a medication.  This may be as simple as swallowing a tablet or it may be a
/// long running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationAdministrationDosage {
/// The amount of the medication given at one administration event.   Use this
/// value when the administration is essentially an instantaneous event such as a
/// swallowing a tablet or giving an injection.
pub dose: Quantity,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A coded value indicating the method by which the medication is intended to be
/// or was introduced into or on the body.  This attribute will most often NOT be
/// populated.  It is most commonly used for injections.  For example, Slow Push,
/// Deep IV.
pub method: CodeableConcept,
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
/// Identifies the speed with which the medication was or will be introduced into
/// the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or
/// 100 ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2
/// hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours.
pub rate_quantity: Quantity,
/// Identifies the speed with which the medication was or will be introduced into
/// the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or
/// 100 ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2
/// hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours.
pub rate_ratio: Ratio,
/// A code specifying the route or physiological path of administration of a
/// therapeutic agent into or onto the patient.  For example, topical, intravenous,
/// etc.
pub route: CodeableConcept,
/// A coded specification of the anatomic site where the medication first entered
/// the body.  For example, "left arm".
pub site: CodeableConcept,
/// Free text dosage can be used for cases where the dosage administered is too
/// complex to code. When coded dosage is present, the free text dosage may still be
/// present for display to humans.
///
/// The dosage instructions should reflect the dosage of the medication that was
/// administered.
pub text: String,
}