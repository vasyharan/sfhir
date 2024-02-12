use super::*;
/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug,Clone,PartialEq)]
pub struct SubscriptionTopicCanFilterBy {
/// Comparators allowed for the filter parameter.
pub comparator: Vec<Code>,
/// Description of how this filtering parameter is intended to be used.
pub description: Markdown,
/// Either the canonical URL to a search parameter (like "http://hl7.org/fhir/
/// SearchParameter/encounter-patient") or the officially-defined URI for a shared
/// filter concept (like "http://example.org/concepts/shared-common-event").
pub filter_definition: Uri,
/// Either the canonical URL to a search parameter (like "http://hl7.org/
/// fhir/SearchParameter/encounter-patient") or topic-defined parameter (like
/// "hub.event") which is a label for the filter.
pub filter_parameter: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Modifiers allowed for the filter parameter.
pub modifier: Vec<Code>,
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
/// URL of the Resource that is the type used in this filter. This is the "focus"
/// of the topic (or one of them if there are more than one). It will be the same,
/// a generality, or a specificity of SubscriptionTopic.resourceTrigger.resource or
/// SubscriptionTopic.eventTrigger.resource when they are present.
pub resource: Uri,
}
