use super::*;
/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance context.
#[derive(Debug,Clone,PartialEq)]
pub struct ActivityDefinitionDynamicValue {
/// An expression specifying the value of the customized element.
pub expression: Expression,
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
/// The path to the element to be customized. This is the path on the resource
/// that will hold the result of the calculation defined by the expression. The
/// specified path SHALL be a FHIRPath resolvable on the specified target type
/// of the ActivityDefinition, and SHALL consist only of identifiers, constant
/// indexers, and a restricted subset of functions. The path is allowed to
/// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
/// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile]
/// (fhirpath.html#simple) for full details).
pub path: String,
}