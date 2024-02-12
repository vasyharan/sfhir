use super::*;
/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug,Clone,PartialEq)]
pub struct SubscriptionTopicQueryCriteria {
/// The FHIR query based rules are applied to the current resource state (e.g.,
/// state after an update).
pub current: String,
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
/// The FHIR query based rules are applied to the previous resource state (e.g.,
/// state before an update).
pub previous: String,
/// If set to `true`, both the `current` and `previous` query criteria must evaluate
/// `true` to trigger a notification for this topic.  If set to `false` or not
/// present, a notification for this topic will be triggered if either the `current`
/// or `previous` tests evaluate to `true`.
pub require_both: Boolean,
/// For `create` interactions, should the `previous` criteria count as an automatic
/// pass or an automatic fail. If not present, the testing behavior during `create`
/// interactions is unspecified (server discretion).
pub result_for_create: Code,
/// For 'delete' interactions, should the 'current' query criteria count as an
/// automatic pass or an automatic fail. If not present, the testing behavior during
/// `delete` interactions is unspecified (server discretion).
pub result_for_delete: Code,
}
