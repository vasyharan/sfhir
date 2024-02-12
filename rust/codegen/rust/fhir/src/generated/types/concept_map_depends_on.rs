use super::*;
/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug,Clone,PartialEq)]
pub struct ConceptMapDependsOn {
/// A reference to the additional attribute that holds a value the map depends on.
pub attribute: Code,
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
/// Data element value that the map depends on / produces.
pub value_boolean: bool,
/// Data element value that the map depends on / produces.
pub value_code: String,
/// Data element value that the map depends on / produces.
pub value_coding: Coding,
/// Data element value that the map depends on / produces.
pub value_quantity: Quantity,
/// This mapping applies if the data element value is a code from this value set.
pub value_set: Canonical,
/// Data element value that the map depends on / produces.
pub value_string: String,
}
