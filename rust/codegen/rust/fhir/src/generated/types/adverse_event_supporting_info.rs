use super::*;
/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug,Clone,PartialEq)]
pub struct AdverseEventSupportingInfo {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Relevant past history for the subject. In a clinical care context, an example
/// being a patient had an adverse event following a pencillin administration and
/// the patient had a previously documented penicillin allergy. In a clinical trials
/// context, an example is a bunion or rash that was present prior to the study.
/// Additionally, the supporting item can be a document that is relevant to this
/// instance of the adverse event that is not part of the subject's medical history.
/// For example, a clinical note, staff list, or material safety data sheet (MSDS).
/// Supporting information is not a contributing factor, preventive action, or
/// mitigating action.
pub item_codeable_concept: CodeableConcept,
/// Relevant past history for the subject. In a clinical care context, an example
/// being a patient had an adverse event following a pencillin administration and
/// the patient had a previously documented penicillin allergy. In a clinical trials
/// context, an example is a bunion or rash that was present prior to the study.
/// Additionally, the supporting item can be a document that is relevant to this
/// instance of the adverse event that is not part of the subject's medical history.
/// For example, a clinical note, staff list, or material safety data sheet (MSDS).
/// Supporting information is not a contributing factor, preventive action, or
/// mitigating action.
pub item_reference: Reference,
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