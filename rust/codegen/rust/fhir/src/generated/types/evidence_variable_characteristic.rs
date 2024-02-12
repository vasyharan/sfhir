use super::*;
/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug,Clone,PartialEq)]
pub struct EvidenceVariableCharacteristic {
/// Defines the characteristic as a combination of two or more characteristics.
pub definition_by_combination: EvidenceVariableDefinitionByCombination,
/// Defines the characteristic using both a type and value[x] elements.
pub definition_by_type_and_value: EvidenceVariableDefinitionByTypeAndValue,
/// Defines the characteristic using Canonical.
pub definition_canonical: Canonical,
/// Defines the characteristic using CodeableConcept.
pub definition_codeable_concept: CodeableConcept,
/// Defines the characteristic using Expression.
pub definition_expression: Expression,
/// Defines the characteristic using id.
pub definition_id: Id,
/// Defines the characteristic using a Reference.
pub definition_reference: Reference,
/// A short, natural language description of the characteristic that could be used
/// to communicate the criteria to an end-user.
pub description: Markdown,
/// Length of time in which the characteristic is met.
pub duration_quantity: Quantity,
/// Length of time in which the characteristic is met.
pub duration_range: Range,
/// When true, this characteristic is an exclusion criterion. In other words, not
/// matching this characteristic definition is equivalent to meeting this criterion.
pub exclude: Boolean,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Number of occurrences meeting the characteristic.
pub instances_quantity: Quantity,
/// Number of occurrences meeting the characteristic.
pub instances_range: Range,
/// Label used for when a characteristic refers to another characteristic.
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
/// A human-readable string to clarify or explain concepts about the characteristic.
pub note: Vec<Annotation>,
/// Timing in which the characteristic is determined.
pub time_from_event: Vec<EvidenceVariableTimeFromEvent>,
}
