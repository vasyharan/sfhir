use super::*;
/// The Measure resource provides the definition of a quality measure.
#[derive(Debug,Clone,PartialEq)]
pub struct MeasurePopulation {
/// Specifies which method should be used to aggregate measure observation values.
/// For most scoring types, this is implied by scoring (e.g. a proportion measure
/// counts members of the populations). For continuous variables, however, this
/// information must be specified to ensure correct calculation.
pub aggregate_method: CodeableConcept,
/// The type of population criteria.
pub code: CodeableConcept,
/// An expression that specifies the criteria for the population, typically the name
/// of an expression in a library.
pub criteria: Expression,
/// The human readable description of this population criteria.
pub description: Markdown,
/// A Group resource that defines this population as a set of characteristics.
pub group_definition: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The id of a population element in this measure that provides the input for this
/// population criteria. In most cases, the scoring structure of the measure implies
/// specific relationships (e.g. the Numerator uses the Denominator as the source in
/// a proportion scoring). In some cases, however, multiple possible choices exist
/// and must be resolved explicitly. For example in a ratio measure with multiple
/// initial populations, the denominator must specify which population should be
/// used as the starting point.
pub input_population_id: String,
/// An identifier that is unique within the Measure allowing linkage to the
/// equivalent population in a MeasureReport resource.
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
}
