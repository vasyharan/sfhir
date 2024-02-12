use super::*;
/// Information about a medication that is used to support knowledge.
#[derive(Debug,Clone,PartialEq)]
pub struct MedicationKnowledgeDrugCharacteristic {
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
/// A code specifying which characteristic of the medicine is being described (for
/// example, colour, shape, imprint).
pub r#type: CodeableConcept,
/// Description of the characteristic.
pub value_attachment: Attachment,
/// Description of the characteristic.
pub value_base_64_binary: String,
/// Description of the characteristic.
pub value_codeable_concept: CodeableConcept,
/// Description of the characteristic.
pub value_quantity: Quantity,
/// Description of the characteristic.
pub value_string: String,
}
