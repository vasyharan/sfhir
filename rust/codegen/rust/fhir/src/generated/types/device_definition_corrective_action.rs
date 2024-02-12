use super::*;
/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug,Clone,PartialEq)]
pub struct DeviceDefinitionCorrectiveAction {
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
/// Start and end dates of the  corrective action.
pub period: Period,
/// Whether the last corrective action known for this device was a recall.
pub recall: Boolean,
/// The scope of the corrective action - whether the action targeted all units of a
/// given device model, or only a specific set of batches identified by lot numbers,
/// or individually identified devices identified by the serial name.
pub scope: Code,
}
