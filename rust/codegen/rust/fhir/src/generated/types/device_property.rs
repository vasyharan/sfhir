use super::*;
/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug,Clone,PartialEq)]
pub struct DeviceProperty {
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
/// Code that specifies the property, such as resolution, color, size, being
/// represented.
pub r#type: CodeableConcept,
/// The value of the property specified by the associated property.type code.
pub value_attachment: Attachment,
/// The value of the property specified by the associated property.type code.
pub value_boolean: bool,
/// The value of the property specified by the associated property.type code.
pub value_codeable_concept: CodeableConcept,
/// The value of the property specified by the associated property.type code.
pub value_integer: i64,
/// The value of the property specified by the associated property.type code.
pub value_quantity: Quantity,
/// The value of the property specified by the associated property.type code.
pub value_range: Range,
/// The value of the property specified by the associated property.type code.
pub value_string: String,
}
