use super::*;
/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder"
/// to generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationRequestInitialFill {
/// The length of time that the first dispense is expected to last.
pub duration: Duration,
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
/// The amount or quantity to provide as part of the first dispense.
pub quantity: Quantity,
}