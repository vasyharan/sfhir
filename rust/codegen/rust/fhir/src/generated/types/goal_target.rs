use super::*;
/// Describes the intended objective(s) for a patient, group or organization care,
/// for example, weight loss, restoring an activity of daily living, obtaining herd
/// immunity via immunization, meeting a process improvement objective, etc.
#[derive(Debug,Clone,PartialEq)]
pub struct GoalTarget {
/// The target value of the focus to be achieved to signify the fulfillment of the
/// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
/// can be specified. When a low value is missing, it indicates that the goal is
/// achieved at any focus value at or below the high value. Similarly, if the high
/// value is missing, it indicates that the goal is achieved at any focus value at
/// or above the low value.
pub detail_boolean: bool,
/// The target value of the focus to be achieved to signify the fulfillment of the
/// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
/// can be specified. When a low value is missing, it indicates that the goal is
/// achieved at any focus value at or below the high value. Similarly, if the high
/// value is missing, it indicates that the goal is achieved at any focus value at
/// or above the low value.
pub detail_codeable_concept: CodeableConcept,
/// The target value of the focus to be achieved to signify the fulfillment of the
/// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
/// can be specified. When a low value is missing, it indicates that the goal is
/// achieved at any focus value at or below the high value. Similarly, if the high
/// value is missing, it indicates that the goal is achieved at any focus value at
/// or above the low value.
pub detail_integer: i64,
/// The target value of the focus to be achieved to signify the fulfillment of the
/// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
/// can be specified. When a low value is missing, it indicates that the goal is
/// achieved at any focus value at or below the high value. Similarly, if the high
/// value is missing, it indicates that the goal is achieved at any focus value at
/// or above the low value.
pub detail_quantity: Quantity,
/// The target value of the focus to be achieved to signify the fulfillment of the
/// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
/// can be specified. When a low value is missing, it indicates that the goal is
/// achieved at any focus value at or below the high value. Similarly, if the high
/// value is missing, it indicates that the goal is achieved at any focus value at
/// or above the low value.
pub detail_range: Range,
/// The target value of the focus to be achieved to signify the fulfillment of the
/// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
/// can be specified. When a low value is missing, it indicates that the goal is
/// achieved at any focus value at or below the high value. Similarly, if the high
/// value is missing, it indicates that the goal is achieved at any focus value at
/// or above the low value.
pub detail_ratio: Ratio,
/// The target value of the focus to be achieved to signify the fulfillment of the
/// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
/// can be specified. When a low value is missing, it indicates that the goal is
/// achieved at any focus value at or below the high value. Similarly, if the high
/// value is missing, it indicates that the goal is achieved at any focus value at
/// or above the low value.
pub detail_string: String,
/// Indicates either the date or the duration after start by which the goal should
/// be met.
pub due_date: String,
/// Indicates either the date or the duration after start by which the goal should
/// be met.
pub due_duration: Duration,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The parameter whose value is being tracked, e.g. body weight, blood pressure, or
/// hemoglobin A1c level.
pub measure: CodeableConcept,
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
}
