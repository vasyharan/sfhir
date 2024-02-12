use super::*;
/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug,Clone,PartialEq)]
pub struct EvidenceStatistic {
/// A statistical attribute of the statistic such as a measure of heterogeneity.
pub attribute_estimate: Vec<EvidenceAttributeEstimate>,
/// When the measured variable is handled categorically, the category element is
/// used to define which category the statistic is reporting.
pub category: CodeableConcept,
/// A description of the content value of the statistic.
pub description: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A component of the method to generate the statistic.
pub model_characteristic: Vec<EvidenceModelCharacteristic>,
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
/// Footnotes and/or explanatory notes.
pub note: Vec<Annotation>,
/// The number of participants affected where the unit of analysis is the same as
/// sampleSize.knownDataCount and sampleSize.numberOfParticipants.
pub number_affected: UnsignedInt,
/// The number of events associated with the statistic, where the unit of
/// analysis is different from numberAffected, sampleSize.knownDataCount and
/// sampleSize.numberOfParticipants.
pub number_of_events: UnsignedInt,
/// Statistic value.
pub quantity: Quantity,
/// Number of samples in the statistic.
pub sample_size: EvidenceSampleSize,
/// Type of statistic, e.g., relative risk.
pub statistic_type: CodeableConcept,
}
