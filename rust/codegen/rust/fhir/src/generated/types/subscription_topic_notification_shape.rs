use super::*;
/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug,Clone,PartialEq)]
pub struct SubscriptionTopicNotificationShape {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Search-style _include directives, rooted in the resource for this shape. Servers
/// SHOULD include resources listed here, if they exist and the user is authorized
/// to receive them.  Clients SHOULD be prepared to receive these additional
/// resources, but SHALL function properly without them.
pub include: Vec<String>,
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
/// URL of the Resource that is the type used in this shape. This is the
/// 'focus' resource of the topic (or one of them if there are more than one)
/// and the root resource for this shape definition. It will be the same, a
/// generality, or a specificity of SubscriptionTopic.resourceTrigger.resource or
/// SubscriptionTopic.eventTrigger.resource when they are present.
pub resource: Uri,
/// Search-style _revinclude directives, rooted in the resource for this shape.
/// Servers SHOULD include resources listed here, if they exist and the user
/// is authorized to receive them.  Clients SHOULD be prepared to receive these
/// additional resources, but SHALL function properly without them.
pub rev_include: Vec<String>,
}