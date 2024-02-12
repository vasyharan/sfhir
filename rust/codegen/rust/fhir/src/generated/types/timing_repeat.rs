use super::*;
/// Specifies an event that may occur multiple times. Timing schedules are used
/// to record when things are planned, expected or requested to occur. The most
/// common usage is in dosage instructions for medications. They are also used when
/// planning care of various kinds, and may be used for reporting the schedule to
/// which past regular activities were carried out.
#[derive(Debug,Clone,PartialEq)]
pub struct TimingRepeat {
/// Either a duration for the length of the timing schedule, a range of possible
/// length, or outer bounds for start and/or end limits of the timing schedule.
pub bounds_duration: Duration,
/// Either a duration for the length of the timing schedule, a range of possible
/// length, or outer bounds for start and/or end limits of the timing schedule.
pub bounds_period: Period,
/// Either a duration for the length of the timing schedule, a range of possible
/// length, or outer bounds for start and/or end limits of the timing schedule.
pub bounds_range: Range,
/// A total count of the desired number of repetitions across the duration of the
/// entire timing specification. If countMax is present, this element indicates the
/// lower bound of the allowed range of count values.
pub count: PositiveInt,
/// If present, indicates that the count is a range - so to perform the action
/// between [count] and [countMax] times.
pub count_max: PositiveInt,
/// If one or more days of week is provided, then the action happens only on the
/// specified day(s).
pub day_of_week: Vec<Code>,
/// How long this thing happens for when it happens. If durationMax is present, this
/// element indicates the lower bound of the allowed range of the duration.
pub duration: Decimal,
/// If present, indicates that the duration is a range - so to perform the action
/// between [duration] and [durationMax] time length.
pub duration_max: Decimal,
/// The units of time for the duration, in UCUM units
/// Normal practice is to use the 'mo' code as a calendar month when calculating the
/// next occurrence.
pub duration_unit: DurationUnit,
/// The number of times to repeat the action within the specified period. If
/// frequencyMax is present, this element indicates the lower bound of the allowed
/// range of the frequency.
pub frequency: PositiveInt,
/// If present, indicates that the frequency is a range - so to repeat between
/// [frequency] and [frequencyMax] times within the period or period range.
pub frequency_max: PositiveInt,
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
/// The number of minutes from the event. If the event code does not indicate
/// whether the minutes is before or after the event, then the offset is assumed to
/// be after the event.
pub offset: UnsignedInt,
/// Indicates the duration of time over which repetitions are to occur; e.g. to
/// express "3 times per day", 3 would be the frequency and "1 day" would be the
/// period. If periodMax is present, this element indicates the lower bound of the
/// allowed range of the period length.
pub period: Decimal,
/// If present, indicates that the period is a range from [period] to [periodMax],
/// allowing expressing concepts such as "do this once every 3-5 days.
pub period_max: Decimal,
/// The units of time for the period in UCUM units
/// Normal practice is to use the 'mo' code as a calendar month when calculating the
/// next occurrence.
pub period_unit: PeriodUnit,
/// Specified time of day for action to take place.
pub time_of_day: Vec<Time>,
/// An approximate time period during the day, potentially linked to an event of
/// daily living that indicates when the action should occur.
pub when: Vec<When>,
}

#[derive(Debug,Clone,PartialEq)]
pub enum DurationUnit {
S,
Min,
H,
D,
Wk,
Mo,
A,
}

#[derive(Debug,Clone,PartialEq)]
pub enum PeriodUnit {
S,
Min,
H,
D,
Wk,
Mo,
A,
}
