use super::*;
/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug,Clone,PartialEq)]
pub struct ObservationDefinitionComponent {
/// Describes what will be observed.
pub code: CodeableConcept,
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
/// The data types allowed for the value element of the instance of this component
/// observations.
pub permitted_data_type: Vec<Code>,
/// Units allowed for the valueQuantity element in the instance observations
/// conforming to this ObservationDefinition.
pub permitted_unit: Vec<Coding>,
/// A set of qualified values associated with a context and a set of conditions -
/// provides a range for quantitative and ordinal observations and a collection of
/// value sets for qualitative observations.
pub qualified_value: Vec<ObservationDefinitionQualifiedValue>,
}
