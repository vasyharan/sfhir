use super::*;
/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug,Clone,PartialEq)]
pub struct MeasureReportStratum {
/// A stratifier component value.
pub component: Vec<MeasureReportComponent>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The measure score for this stratum, calculated as appropriate for the measure
/// type and scoring method, and based on only the members of this stratum.
pub measure_score_codeable_concept: CodeableConcept,
/// The measure score for this stratum, calculated as appropriate for the measure
/// type and scoring method, and based on only the members of this stratum.
pub measure_score_date_time: String,
/// The measure score for this stratum, calculated as appropriate for the measure
/// type and scoring method, and based on only the members of this stratum.
pub measure_score_duration: Duration,
/// The measure score for this stratum, calculated as appropriate for the measure
/// type and scoring method, and based on only the members of this stratum.
pub measure_score_period: Period,
/// The measure score for this stratum, calculated as appropriate for the measure
/// type and scoring method, and based on only the members of this stratum.
pub measure_score_quantity: Quantity,
/// The measure score for this stratum, calculated as appropriate for the measure
/// type and scoring method, and based on only the members of this stratum.
pub measure_score_range: Range,
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
/// The populations that make up the stratum, one for each type of population
/// appropriate to the measure.
pub population: Vec<MeasureReportPopulation1>,
/// The value for this stratum, expressed as a CodeableConcept. When defining
/// stratifiers on complex values, the value must be rendered such that the value
/// for each stratum within the stratifier is unique.
pub value_boolean: bool,
/// The value for this stratum, expressed as a CodeableConcept. When defining
/// stratifiers on complex values, the value must be rendered such that the value
/// for each stratum within the stratifier is unique.
pub value_codeable_concept: CodeableConcept,
/// The value for this stratum, expressed as a CodeableConcept. When defining
/// stratifiers on complex values, the value must be rendered such that the value
/// for each stratum within the stratifier is unique.
pub value_quantity: Quantity,
/// The value for this stratum, expressed as a CodeableConcept. When defining
/// stratifiers on complex values, the value must be rendered such that the value
/// for each stratum within the stratifier is unique.
pub value_range: Range,
/// The value for this stratum, expressed as a CodeableConcept. When defining
/// stratifiers on complex values, the value must be rendered such that the value
/// for each stratum within the stratifier is unique.
pub value_reference: Reference,
}
