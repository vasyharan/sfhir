use super::*;
/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.
#[derive(Debug,Clone,PartialEq)]
pub struct CarePlanActivity {
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
/// Identifies the activity that was performed. For example, an activity could be
/// patient education, exercise, or a medication administration. The reference to
/// an "event" resource, such as Procedure or Encounter or Observation, represents
/// the activity that was performed. The requested activity can be conveyed using
/// the CarePlan.activity.plannedActivityReference (a reference to a “request”
/// resource).
pub performed_activity: Vec<CodeableReference>,
/// The details of the proposed activity represented in a specific resource.
pub planned_activity_reference: Reference,
/// Notes about the adherence/status/progress of the activity.
pub progress: Vec<Annotation>,
}
