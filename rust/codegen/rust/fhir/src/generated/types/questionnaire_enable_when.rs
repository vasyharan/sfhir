use super::*;
/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug,Clone,PartialEq)]
pub struct QuestionnaireEnableWhen {
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_boolean: bool,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_coding: Coding,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_date: String,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_date_time: String,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_decimal: f64,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_integer: i64,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_quantity: Quantity,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_reference: Reference,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_string: String,
/// A value that the referenced question is tested using the specified operator in
/// order for the item to be enabled.  If there are multiple answers, a match on any
/// of the answers suffices.  If different behavior is desired (all must match, at
/// least 2 must match, etc.), consider using the enableWhenExpression extension.
pub answer_time: String,
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
/// Specifies the criteria by which the question is enabled.
pub operator: Code,
/// The linkId for the question whose answer (or lack of answer) governs whether
/// this item is enabled.
pub question: String,
}
