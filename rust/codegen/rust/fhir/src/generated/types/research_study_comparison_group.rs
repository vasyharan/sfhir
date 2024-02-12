use super::*;
/// A scientific study of nature that sometimes includes processes involved in
/// health and disease. For example, clinical trials are research studies that
/// involve people. These studies may be related to new ways to screen, prevent,
/// diagnose, and treat disease. They may also study certain outcomes and certain
/// groups of people by looking at data collected in the past or future.
#[derive(Debug,Clone,PartialEq)]
pub struct ResearchStudyComparisonGroup {
/// A succinct description of the path through the study that would be followed by a
/// subject adhering to this comparisonGroup.
pub description: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Interventions or exposures in this comparisonGroup or cohort.
pub intended_exposure: Vec<Reference>,
/// Allows the comparisonGroup for the study and the comparisonGroup for the subject
/// to be linked easily.
pub link_id: Id,
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
/// Unique, human-readable label for this comparisonGroup of the study.
pub name: String,
/// Group of participants who were enrolled in study comparisonGroup.
pub observed_group: Reference,
/// Categorization of study comparisonGroup, e.g. experimental, active comparator,
/// placebo comparater.
pub r#type: CodeableConcept,
}
