use super::*;
/// A patient's point-in-time set of recommendations (i.e. forecasting) according to
/// a published schedule with optional supporting justification.
#[derive(Debug,Clone,PartialEq)]
pub struct ImmunizationRecommendationRecommendation {
/// Vaccine(s) which should not be used to fulfill the recommendation.
pub contraindicated_vaccine_code: Vec<CodeableConcept>,
/// Vaccine date recommendations.  For example, earliest date to administer, latest
/// date to administer, etc.
pub date_criterion: Vec<ImmunizationRecommendationDateCriterion>,
/// Contains the description about the protocol under which the vaccine was
/// administered.
pub description: Markdown,
/// Nominal position of the recommended dose in a series as determined by the
/// evaluation and forecasting process (e.g. dose 2 is the next recommended dose).
pub dose_number: String,
/// The reason for the assigned forecast status.
pub forecast_reason: Vec<CodeableConcept>,
/// Indicates the patient status with respect to the path to immunity for the target
/// disease.
pub forecast_status: CodeableConcept,
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
/// One possible path to achieve presumed immunity against a disease - within the
/// context of an authority.
pub series: String,
/// The recommended number of doses to achieve immunity as determined by the
/// evaluation and forecasting process.
pub series_doses: String,
/// Immunization event history and/or evaluation that supports the status and
/// recommendation.
pub supporting_immunization: Vec<Reference>,
/// Patient Information that supports the status and recommendation.  This includes
/// patient observations, adverse reactions and allergy/intolerance information.
pub supporting_patient_information: Vec<Reference>,
/// The targeted disease for the recommendation.
pub target_disease: Vec<CodeableConcept>,
/// Vaccine(s) or vaccine group that pertain to the recommendation.
pub vaccine_code: Vec<CodeableConcept>,
}
