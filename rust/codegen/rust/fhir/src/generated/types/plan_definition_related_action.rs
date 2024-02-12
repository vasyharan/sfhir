use super::*;
/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug,Clone,PartialEq)]
pub struct PlanDefinitionRelatedAction {
/// The relationship of the end of this action to the related action.
pub end_relationship: Code,
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
/// A duration or range of durations to apply to the relationship. For example, 30-
/// 60 minutes before.
pub offset_duration: Duration,
/// A duration or range of durations to apply to the relationship. For example, 30-
/// 60 minutes before.
pub offset_range: Range,
/// The relationship of the start of this action to the related action.
pub relationship: Code,
/// The element id of the target related action.
pub target_id: Id,
}
