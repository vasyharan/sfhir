use super::*;
/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug,Clone,PartialEq)]
pub struct AppointmentMonthlyTemplate {
/// Indicates that appointments in the series of recurring appointments should occur
/// on a specific day of the month.
pub day_of_month: PositiveInt,
/// Indicates which day of the week the recurring appointments should occur each
/// nth week.
pub day_of_week: Coding,
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
/// Indicates that recurring appointments should occur every nth month.
pub month_interval: PositiveInt,
/// Indicates which week within a month the appointments in the series of recurring
/// appointments should occur on.
pub nth_week_of_month: Coding,
}
