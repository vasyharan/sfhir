use super::*;
/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug,Clone,PartialEq)]
pub struct MeasureReportPopulation {
/// The type of the population.
pub code: CodeableConcept,
/// The number of members of the population.
pub count: Integer,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The population from the Measure that corresponds to this population in the
/// MeasureReport resource.
pub link_id: String,
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
/// A reference to an individual level MeasureReport resource for a member of the
/// population.
pub subject_report: Vec<Reference>,
/// This element refers to a List of individual level MeasureReport resources, one
/// for each subject in this population.
pub subject_results: Reference,
/// Optional Group identifying the individuals that make up the population.
pub subjects: Reference,
}