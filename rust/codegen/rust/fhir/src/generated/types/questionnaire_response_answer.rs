use super::*;
/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.
#[derive(Debug,Clone,PartialEq)]
pub struct QuestionnaireResponseAnswer {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Nested groups and/or questions found within this particular answer.
pub item: Vec<QuestionnaireResponseItem>,
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
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_attachment: Attachment,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_boolean: bool,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_coding: Coding,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_date: String,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_date_time: String,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_decimal: f64,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_integer: i64,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_quantity: Quantity,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_reference: Reference,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_string: String,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_time: String,
/// The answer (or one of the answers) provided by the respondent to the question.
pub value_uri: String,
}
