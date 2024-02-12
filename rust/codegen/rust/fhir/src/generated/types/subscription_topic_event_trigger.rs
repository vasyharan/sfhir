use super::*;
/// Describes a stream of resource state changes or events and annotated with labels
/// useful to filter projections from this topic.
#[derive(Debug,Clone,PartialEq)]
pub struct SubscriptionTopicEventTrigger {
/// The human readable description of an event to trigger a notification for the
/// SubscriptionTopic - for example, "Patient Admission, as defined in HL7v2 via
/// message ADT^A01". Multiple values are considered OR joined (e.g., matching any
/// single event listed).
pub description: Markdown,
/// A well-defined event which can be used to trigger notifications from the
/// SubscriptionTopic.
pub event: CodeableConcept,
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
/// URL of the Resource that is the focus type used in this event trigger.
/// Relative URLs are relative to the StructureDefinition root of the
/// implemented FHIR version (e.g., http://hl7.org/fhir/StructureDefinition).
/// For example, "Patient" maps to http://hl7.org/fhir/StructureDefinition/
/// Patient.  For more information, see <a href="elementdefinition-
/// definitions.html#ElementDefinition.type.code">ElementDefinition.type.code</a>.
pub resource: Uri,
}
