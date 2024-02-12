use super::*;
/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug,Clone,PartialEq)]
pub struct ConceptMapTarget {
/// Identity (code or path) or the element/item that the map refers to.
pub code: Code,
/// A description of status/issues in mapping that conveys additional information
/// not represented in  the structured data.
pub comment: String,
/// A set of additional dependencies for this mapping to hold. This mapping is
/// only applicable if the specified data attribute can be resolved, and it has the
/// specified value.
pub depends_on: Vec<ConceptMapDependsOn>,
/// The display for the code. The display is only provided to help editors when
/// editing the concept map.
pub display: String,
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
/// Product is the output of a ConceptMap that provides additional values that go in
/// other attributes / data elemnts of the target data.
pub product: Vec<ConceptMapDependsOn>,
/// A property value for this source -> target mapping.
pub property: Vec<ConceptMapProperty1>,
/// The relationship between the source and target concepts. The relationship is
/// read from source to target (e.g. source-is-narrower-than-target).
pub relationship: Code,
/// The set of concepts from the ConceptMap.group.target code system which are all
/// being mapped to as part of this mapping rule. The effect of using this data
/// element is the same as having multiple ConceptMap.group.element.target elements
/// with one for each concept in the ConceptMap.group.element.target.valueSet value
/// set.
pub value_set: Canonical,
}
