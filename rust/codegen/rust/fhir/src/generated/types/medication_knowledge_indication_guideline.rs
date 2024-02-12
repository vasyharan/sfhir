use super::*;
/// Information about a medication that is used to support knowledge.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationKnowledgeIndicationGuideline {
/// The guidelines for the dosage of the medication for the indication.
pub dosing_guideline: Vec<MedicationKnowledgeDosingGuideline>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Indication or reason for use of the medication that applies to the specific
/// administration guideline.
pub indication: Vec<CodeableReference>,
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
