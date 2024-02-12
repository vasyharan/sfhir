use super::*;
/// A description of a triggering event. Triggering events can be named events, data
/// events, or periodic, as determined by the type element.
#[derive(Debug,Clone,PartialEq)]
pub struct TriggerDefinition {
/// A code that identifies the event.
pub code: CodeableConcept,
/// A boolean-valued expression that is evaluated in the context of the container of
/// the trigger definition and returns whether or not the trigger fires.
pub condition: Expression,
/// The triggering data of the event (if this is a data trigger). If more than one
/// data is requirement is specified, then all the data requirements must be true.
pub data: Vec<DataRequirement>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A formal name for the event. This may be an absolute URI that identifies the
/// event formally (e.g. from a trigger registry), or a simple relative URI that
/// identifies the event in a local context.
pub name: String,
/// A reference to a SubscriptionTopic resource that defines the event. If this
/// element is provided, no other information about the trigger definition may be
/// supplied.
pub subscription_topic: Canonical,
/// The timing of the event (if this is a periodic trigger).
pub timing_date: String,
/// The timing of the event (if this is a periodic trigger).
pub timing_date_time: String,
/// The timing of the event (if this is a periodic trigger).
pub timing_reference: Reference,
/// The timing of the event (if this is a periodic trigger).
pub timing_timing: Timing,
/// The type of triggering event.
pub r#type: Type,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Type {
NamedEvent,
Periodic,
DataChanged,
DataAdded,
DataModified,
DataRemoved,
DataAccessed,
DataAccessEnded,
}
