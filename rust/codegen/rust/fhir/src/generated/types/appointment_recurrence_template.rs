use super::*;
/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug,Clone,PartialEq)]
pub struct AppointmentRecurrenceTemplate {
/// Any dates, such as holidays, that should be excluded from the recurrence.
pub excluding_date: Vec<Date>,
/// Any dates, such as holidays, that should be excluded from the recurrence.
pub excluding_recurrence_id: Vec<PositiveInt>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Recurring appointments will not occur after this date.
pub last_occurrence_date: Date,
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
/// Information about monthly recurring appointments.
pub monthly_template: AppointmentMonthlyTemplate,
/// How many appointments are planned in the recurrence.
pub occurrence_count: PositiveInt,
/// The list of specific dates that will have appointments generated.
pub occurrence_date: Vec<Date>,
/// How often the appointment series should recur.
pub recurrence_type: CodeableConcept,
/// The timezone of the recurring appointment occurrences.
pub timezone: CodeableConcept,
/// Information about weekly recurring appointments.
pub weekly_template: AppointmentWeeklyTemplate,
/// Information about yearly recurring appointments.
pub yearly_template: AppointmentYearlyTemplate,
}
