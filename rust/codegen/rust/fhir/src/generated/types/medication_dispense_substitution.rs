use super::*;
/// Indicates that a medication product is to be or has been dispensed for a named
/// person/patient.  This includes a description of the medication product (supply)
/// provided and the instructions for administering the medication.  The medication
/// dispense is the result of a pharmacy system responding to a medication order.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationDispenseSubstitution {
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
/// Indicates the reason for the substitution (or lack of substitution) from what
/// was prescribed.
pub reason: Vec<CodeableConcept>,
/// The person or organization that has primary responsibility for the substitution.
pub responsible_party: Reference,
/// A code signifying whether a different drug was dispensed from what was
/// prescribed.
pub r#type: CodeableConcept,
/// True if the dispenser dispensed a different drug or product from what was
/// prescribed.
pub was_substituted: Boolean,
}