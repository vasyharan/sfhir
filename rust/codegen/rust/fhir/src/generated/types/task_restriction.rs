use super::*;
/// A task to be performed.
#[derive(Debug,Clone,PartialEq)]
pub struct TaskRestriction {
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
/// The time-period for which fulfillment is sought. This must fall within
/// the overall time period authorized in the referenced request.  E.g.
/// ServiceRequest.occurance[x].
pub period: Period,
/// For requests that are targeted to more than one potential recipient/target, to
/// identify who is fulfillment is sought for.
pub recipient: Vec<Reference>,
/// Indicates the number of times the requested action should occur.
pub repetitions: PositiveInt,
}
