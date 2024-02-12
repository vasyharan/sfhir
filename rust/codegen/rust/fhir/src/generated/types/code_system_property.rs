use super::*;
/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.
#[derive(Debug,Clone,PartialEq)]
pub struct CodeSystemProperty {
/// A code that is used to identify the property. The code is used internally (in
/// CodeSystem.concept.property.code) and also externally, such as in property
/// filters.
pub code: Code,
/// A description of the property- why it is defined, and how its value might be
/// used.
pub description: String,
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
/// The type of the property value. Properties of type "code" contain a code defined
/// by the code system (e.g. a reference to another defined concept).
pub r#type: Code,
/// Reference to the formal meaning of the property. One possible source of meaning
/// is the [Concept Properties](codesystem-concept-properties.html) code system.
pub uri: Uri,
}
