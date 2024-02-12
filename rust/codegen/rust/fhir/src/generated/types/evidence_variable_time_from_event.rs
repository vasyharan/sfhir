use super::*;
/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug,Clone,PartialEq)]
pub struct EvidenceVariableTimeFromEvent {
/// Human readable description.
pub description: Markdown,
/// The event used as a base point (reference point) in time.
pub event_codeable_concept: CodeableConcept,
/// The event used as a base point (reference point) in time.
pub event_date_time: String,
/// The event used as a base point (reference point) in time.
pub event_id: String,
/// The event used as a base point (reference point) in time.
pub event_reference: Reference,
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
/// A human-readable string to clarify or explain concepts about the timeFromEvent.
pub note: Vec<Annotation>,
/// Used to express the observation at a defined amount of time before or after
/// the event.
pub quantity: Quantity,
/// Used to express the observation within a period before and/or after the event.
pub range: Range,
}
