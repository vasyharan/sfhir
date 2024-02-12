use super::*;
/// The subscription resource describes a particular client's request to be notified
/// about a SubscriptionTopic.
#[derive(Debug,Clone,PartialEq)]
pub struct SubscriptionFilterBy {
/// Comparator applied to this filter parameter.
pub comparator: Code,
/// The filter as defined in the `SubscriptionTopic.canFilterBy.filterParameter`
/// element.
pub filter_parameter: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Modifier applied to this filter parameter.
pub modifier: Code,
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
/// A resource listed in the `SubscriptionTopic` this `Subscription` references
/// (`SubscriptionTopic.canFilterBy.resource`). This element can be used to
/// differentiate filters for topics that include more than one resource type.
pub resource_type: Uri,
/// The literal value or resource path as is legal in search - for example,
/// `Patient/123` or `le1950`.
pub value: String,
}
