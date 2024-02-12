use super::*;
/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug,Clone,PartialEq)]
pub struct QuestionnaireInitial {
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
/// The actual value to for an initial answer.
pub value_attachment: Attachment,
/// The actual value to for an initial answer.
pub value_boolean: bool,
/// The actual value to for an initial answer.
pub value_coding: Coding,
/// The actual value to for an initial answer.
pub value_date: String,
/// The actual value to for an initial answer.
pub value_date_time: String,
/// The actual value to for an initial answer.
pub value_decimal: f64,
/// The actual value to for an initial answer.
pub value_integer: i64,
/// The actual value to for an initial answer.
pub value_quantity: Quantity,
/// The actual value to for an initial answer.
pub value_reference: Reference,
/// The actual value to for an initial answer.
pub value_string: String,
/// The actual value to for an initial answer.
pub value_time: String,
/// The actual value to for an initial answer.
pub value_uri: String,
}
