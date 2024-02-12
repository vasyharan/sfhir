use super::*;
/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug,Clone,PartialEq)]
pub struct ElementDefinitionDiscriminator {
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
/// A FHIRPath expression, using [the simple subset of FHIRPath]
/// (fhirpath.html#simple), that is used to identify the element on which
/// discrimination is based.
pub path: String,
/// How the element value is interpreted when discrimination is evaluated.
pub r#type: Type,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Type {
Value,
Exists,
Pattern,
Type,
Profile,
Position,
}
