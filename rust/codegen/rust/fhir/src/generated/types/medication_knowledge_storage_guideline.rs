use super::*;
/// Information about a medication that is used to support knowledge.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationKnowledgeStorageGuideline {
/// Describes a setting/value on the environment for the adequate storage of the
/// medication and other substances.  Environment settings may involve temperature,
/// humidity, or exposure to light.
pub environmental_setting: Vec<MedicationKnowledgeEnvironmentalSetting>,
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
/// Additional notes about the storage.
pub note: Vec<Annotation>,
/// Reference to additional information about the storage guidelines.
pub reference: Uri,
/// Duration that the medication remains stable if the environmentalSetting is
/// respected.
pub stability_duration: Duration,
}
