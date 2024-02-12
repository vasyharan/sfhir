use super::*;
/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug,Clone,PartialEq)]
pub struct DeviceDefinitionGuideline {
/// A specific situation when a device should not be used because it may cause harm.
pub contraindication: Vec<CodeableConcept>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A clinical condition for which the device was designed to be used.
pub indication: Vec<CodeableConcept>,
/// A description of the general purpose or medical use of the device or its
/// function.
pub intended_use: String,
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
/// A source of information or reference for this guideline.
pub related_artifact: Vec<RelatedArtifact>,
/// Detailed written and visual directions for the user on how to use the device.
pub usage_instruction: Markdown,
/// The circumstances that form the setting for using the device.
pub use_context: Vec<UsageContext>,
/// Specific hazard alert information that a user needs to know before using the
/// device.
pub warning: Vec<CodeableConcept>,
}
