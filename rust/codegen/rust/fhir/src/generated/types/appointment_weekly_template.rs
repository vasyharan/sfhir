use super::*;
/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug,Clone,PartialEq)]
pub struct AppointmentWeeklyTemplate {
/// Indicates that recurring appointments should occur on Fridays.
pub friday: Boolean,
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
/// Indicates that recurring appointments should occur on Mondays.
pub monday: Boolean,
/// Indicates that recurring appointments should occur on Saturdays.
pub saturday: Boolean,
/// Indicates that recurring appointments should occur on Sundays.
pub sunday: Boolean,
/// Indicates that recurring appointments should occur on Thursdays.
pub thursday: Boolean,
/// Indicates that recurring appointments should occur on Tuesdays.
pub tuesday: Boolean,
/// Indicates that recurring appointments should occur on Wednesdays.
pub wednesday: Boolean,
/// The interval defines if the recurrence is every nth week. The default is every
/// week, so it is expected that this value will be 2 or more.
///
/// e.g. For recurring every second week this interval would be 2, or every third
/// week the interval would be 3.
pub week_interval: PositiveInt,
}
