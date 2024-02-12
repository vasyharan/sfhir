use super::*;
/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug,Clone,PartialEq)]
pub struct StructureMapTarget {
/// Variable this rule applies to.
pub context: String,
/// Field to create in the context.
pub element: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// If field is a list, how to manage the list.
pub list_mode: Vec<Code>,
/// Internal rule reference for shared list items.
pub list_rule_id: Id,
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
/// Parameters to the transform.
pub parameter: Vec<StructureMapParameter>,
/// How the data is copied / created.
pub transform: Code,
/// Named context for field, if desired, and a field is specified.
pub variable: Id,
}
