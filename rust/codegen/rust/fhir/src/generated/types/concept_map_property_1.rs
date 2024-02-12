use super::*;
/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug,Clone,PartialEq)]
pub struct ConceptMapProperty1 {
/// A reference to a mapping property defined in ConceptMap.property.
pub code: Code,
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
/// The value of this property. If the type chosen for this element is 'code', then
/// the property SHALL be defined in a ConceptMap.property element.
pub value_boolean: bool,
/// The value of this property. If the type chosen for this element is 'code', then
/// the property SHALL be defined in a ConceptMap.property element.
pub value_code: String,
/// The value of this property. If the type chosen for this element is 'code', then
/// the property SHALL be defined in a ConceptMap.property element.
pub value_coding: Coding,
/// The value of this property. If the type chosen for this element is 'code', then
/// the property SHALL be defined in a ConceptMap.property element.
pub value_date_time: String,
/// The value of this property. If the type chosen for this element is 'code', then
/// the property SHALL be defined in a ConceptMap.property element.
pub value_decimal: f64,
/// The value of this property. If the type chosen for this element is 'code', then
/// the property SHALL be defined in a ConceptMap.property element.
pub value_integer: i64,
/// The value of this property. If the type chosen for this element is 'code', then
/// the property SHALL be defined in a ConceptMap.property element.
pub value_string: String,
}
