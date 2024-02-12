use super::*;
/// A food or supplement that is consumed by patients.
#[derive(Debug,Clone,PartialEq)]
pub struct NutritionProductCharacteristic {
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
/// A code specifying which characteristic of the product is being described (for
/// example, colour, shape).
pub r#type: CodeableConcept,
/// The actual characteristic value corresponding to the type.
pub value_attachment: Attachment,
/// The actual characteristic value corresponding to the type.
pub value_base_64_binary: String,
/// The actual characteristic value corresponding to the type.
pub value_boolean: bool,
/// The actual characteristic value corresponding to the type.
pub value_codeable_concept: CodeableConcept,
/// The actual characteristic value corresponding to the type.
pub value_quantity: Quantity,
/// The actual characteristic value corresponding to the type.
pub value_string: String,
}
