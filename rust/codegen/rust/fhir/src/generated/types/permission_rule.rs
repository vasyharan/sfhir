use super::*;
/// Permission resource holds access rules for a given data and context.
#[derive(Debug,Clone,PartialEq)]
pub struct PermissionRule {
/// A description or definition of which activities are allowed to be done on the
/// data.
pub activity: Vec<PermissionActivity>,
/// A description or definition of which activities are allowed to be done on the
/// data.
pub data: Vec<PermissionData>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// What limits apply to the use of the data.
pub limit: Vec<CodeableConcept>,
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
/// deny | permit.
pub r#type: Code,
}
