use super::*;
/// A medicinal product, being a substance or combination of substances that is
/// intended to treat, prevent or diagnose a disease, or to restore, correct or
/// modify physiological functions by exerting a pharmacological, immunological or
/// metabolic action. This resource is intended to define and detail such products
/// and their properties, for uses other than direct patient care (e.g. regulatory
/// use, or drug catalogs).
#[derive(Debug,Clone,PartialEq)]
pub struct MedicinalProductDefinitionCharacteristic {
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
/// A code expressing the type of characteristic.
pub r#type: CodeableConcept,
/// A value for the characteristic.text.
pub value_attachment: Attachment,
/// A value for the characteristic.text.
pub value_boolean: bool,
/// A value for the characteristic.text.
pub value_codeable_concept: CodeableConcept,
/// A value for the characteristic.text.
pub value_date: String,
/// A value for the characteristic.text.
pub value_integer: i64,
/// A value for the characteristic.text.
pub value_markdown: String,
/// A value for the characteristic.text.
pub value_quantity: Quantity,
}