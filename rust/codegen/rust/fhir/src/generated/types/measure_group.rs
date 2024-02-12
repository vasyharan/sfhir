use super::*;
/// The Measure resource provides the definition of a quality measure.
#[derive(Debug,Clone,PartialEq)]
pub struct MeasureGroup {
/// The population basis specifies the type of elements in the population. For a
/// subject-based measure, this is boolean (because the subject and the population
/// basis are the same, and the population criteria define yes/no values for each
/// individual in the population). For measures that have a population basis that
/// is different than the subject, this element specifies the type of the population
/// basis. For example, an encounter-based measure has a subject of Patient and
/// a population basis of Encounter, and the population criteria all return lists
/// of Encounters.
pub basis: Code,
/// Indicates a meaning for the group. This can be as simple as a unique identifier,
/// or it can establish meaning in a broader context by drawing from a terminology,
/// allowing groups to be correlated across measures.
pub code: CodeableConcept,
/// The human readable description of this population group.
pub description: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Information on whether an increase or decrease in score is the preferred result
/// (e.g., a higher score indicates better quality OR a lower score indicates better
/// quality OR quality is within a range).
pub improvement_notation: CodeableConcept,
/// A reference to a Library resource containing the formal logic used by the
/// measure group.
pub library: Vec<Canonical>,
/// An identifier that is unique within the Measure allowing linkage to the
/// equivalent item in a MeasureReport resource.
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
/// A population criteria for the measure.
pub population: Vec<MeasurePopulation>,
/// Describes how to combine the information calculated, based on logic in each of
/// several populations, into one summarized result.
pub rate_aggregation: Markdown,
/// Indicates how the calculation is performed for the measure, including
/// proportion, ratio, continuous-variable, and cohort. The value set is extensible,
/// allowing additional measure scoring types to be represented.
pub scoring: CodeableConcept,
/// Defines the expected units of measure for the measure score. This element SHOULD
/// be specified as a UCUM unit.
pub scoring_unit: CodeableConcept,
/// The stratifier criteria for the measure report, specified as either the name
/// of a valid CQL expression defined within a referenced library or a valid FHIR
/// Resource Path.
pub stratifier: Vec<MeasureStratifier>,
/// The intended subjects for the measure. If this element is not provided, a
/// Patient subject is assumed, but the subject of the measure can be anything.
pub subject_codeable_concept: CodeableConcept,
/// The intended subjects for the measure. If this element is not provided, a
/// Patient subject is assumed, but the subject of the measure can be anything.
pub subject_reference: Reference,
/// Indicates whether the measure is used to examine a process, an outcome over
/// time, a patient-reported outcome, or a structure measure such as utilization.
pub r#type: Vec<CodeableConcept>,
}
