use super::*;
/// The SubscriptionStatus resource describes the state of a Subscription during
/// notifications.
#[derive(Debug,Clone,PartialEq)]
pub struct SubscriptionStatusNotificationEvent {
/// Additional context information for this event. Generally, this will contain
/// references to additional resources included with the event (e.g., the Patient
/// relevant to an Encounter), however it MAY refer to non-FHIR objects.
pub additional_context: Vec<Reference>,
/// Either the sequential number of this event in this subscription context or a
/// relative event number for this notification.
pub event_number: Integer64,
/// The focus of this event. While this will usually be a reference to the focus
/// resource of the event, it MAY contain a reference to a non-FHIR object.
pub focus: Reference,
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
/// The actual time this event occurred on the server.
pub timestamp: Instant,
}