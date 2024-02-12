use super::*;
/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug,Clone,PartialEq)]
pub struct ConceptMapProperty {
/// A code that is used to identify the property. The code is used internally
/// (in ConceptMap.group.element.target.property.code) and also in the $translate
/// operation.
pub code: Code,
/// A description of the property - why it is defined, and how its value might be
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
/// The CodeSystem that defines the codes from which values of type ```code``` in
/// property values.
pub system: Canonical,
/// The type of the property value.
pub r#type: Code,
/// Reference to the formal meaning of the property.
pub uri: Uri,
}
