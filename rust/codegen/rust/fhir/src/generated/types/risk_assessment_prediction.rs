use super::*;
/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.
#[derive(Debug,Clone,PartialEq)]
pub struct RiskAssessmentPrediction {
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
/// One of the potential outcomes for the patient (e.g. remission, death,  a
/// particular condition).
pub outcome: CodeableConcept,
/// Indicates how likely the outcome is (in the specified timeframe).
pub probability_decimal: f64,
/// Indicates how likely the outcome is (in the specified timeframe).
pub probability_range: Range,
/// Indicates how likely the outcome is (in the specified timeframe), expressed as a
/// qualitative value (e.g. low, medium, or high).
pub qualitative_risk: CodeableConcept,
/// Additional information explaining the basis for the prediction.
pub rationale: String,
/// Indicates the risk for this particular subject (with their specific
/// characteristics) divided by the risk of the population in general.  (Numbers
/// greater than 1 = higher risk than the population, numbers less than 1 = lower
/// risk.).
pub relative_risk: Decimal,
/// Indicates the period of time or age range of the subject to which the specified
/// probability applies.
pub when_period: Period,
/// Indicates the period of time or age range of the subject to which the specified
/// probability applies.
pub when_range: Range,
}
