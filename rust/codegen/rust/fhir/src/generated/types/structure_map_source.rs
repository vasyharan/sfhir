use super::*;
/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug,Clone,PartialEq)]
pub struct StructureMapSource {
/// FHIRPath expression  - must be true or the mapping engine throws an error
/// instead of completing.
pub check: String,
/// FHIRPath expression  - must be true or the rule does not apply.
pub condition: String,
/// Type or variable this rule applies to.
pub context: Id,
/// A value to use if there is no existing value in the source object.
pub default_value: String,
/// Optional field for this source.
pub element: String,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// How to handle the list mode for this element.
pub list_mode: Code,
/// A FHIRPath expression which specifies a message to put in the transform log when
/// content matching the source rule is found.
pub log_message: String,
/// Specified maximum cardinality for the element - a number or a "*". This is
/// optional; if present, it acts an implicit check on the input content (* just
/// serves as documentation; it's the default value).
pub max: String,
/// Specified minimum cardinality for the element. This is optional; if present, it
/// acts an implicit check on the input content.
pub min: Integer,
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
/// Specified type for the element. This works as a condition on the mapping - use
/// for polymorphic elements.
pub r#type: String,
/// Named context for field, if a field is specified.
pub variable: Id,
}
