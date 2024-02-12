use super::*;
/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (e.g., population, exposures/
/// interventions, comparators, outcomes, measured variables, confounding
/// variables), the statistics, and the certainty of this evidence.
#[derive(Debug,Clone,PartialEq)]
pub struct EvidenceVariableDefinition {
/// A text description or summary of the variable.
pub description: Markdown,
/// Indication of quality of match between intended variable to actual variable.
pub directness_match: CodeableConcept,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Definition of the intended variable related to the Evidence.
pub intended: Reference,
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
/// Definition of the actual variable related to the statistic(s).
pub observed: Reference,
/// population | subpopulation | exposure | referenceExposure | measuredVariable
/// | confounder.
pub variable_role: CodeableConcept,
}
