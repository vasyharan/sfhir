use super::*;
/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug,Clone,PartialEq)]
pub struct QuestionnaireItem {
/// For items that have a defined set of allowed answers (via answerOption or
/// answerValueSet), indicates whether values *other* than those specified can be
/// selected.
pub answer_constraint: Code,
/// One of the permitted answers for the question.
pub answer_option: Vec<QuestionnaireAnswerOption>,
/// A reference to a value set containing a list of values representing permitted
/// answers for a question.
pub answer_value_set: Canonical,
/// A terminology code that corresponds to this group or question (e.g. a code from
/// LOINC, which defines many questions and answers).
pub code: Vec<Coding>,
/// This element is a URI that refers to an [ElementDefinition]
/// (elementdefinition.html) or to an [ObservationDefinition]
/// (observationdefinition.html) that provides information about this item,
/// including information that might otherwise be included in the instance of the
/// Questionnaire resource. A detailed description of the construction of the URI is
/// shown in [Comments](questionnaire.html#definition), below.
pub definition: Uri,
/// Indicates if and how items that are disabled (because enableWhen evaluates to
/// 'false') should be displayed.
pub disabled_display: Code,
/// Controls how multiple enableWhen values are interpreted -  whether all or any
/// must be true.
pub enable_behavior: Code,
/// A constraint indicating that this item should only be enabled (displayed/allow
/// answers to be captured) when the specified condition is true.
pub enable_when: Vec<QuestionnaireEnableWhen>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// One or more values that should be pre-populated in the answer when initially
/// rendering the questionnaire for user input.
pub initial: Vec<QuestionnaireInitial>,
/// Text, questions and other groups to be nested beneath a question or group.
pub item: Vec<QuestionnaireItem>,
/// An identifier that is unique within the Questionnaire allowing linkage to the
/// equivalent item in a QuestionnaireResponse resource.
pub link_id: String,
/// The maximum number of characters that are permitted in the answer to be
/// considered a "valid" QuestionnaireResponse.
pub max_length: Integer,
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
/// A short label for a particular group, question or set of display text within the
/// questionnaire used for reference by the individual completing the questionnaire.
pub prefix: String,
/// An indication, when true, that the value cannot be changed by a human respondent
/// to the Questionnaire.
pub read_only: Boolean,
/// An indication, if true, that a QuestionnaireResponse for this item may include
/// multiple answers associated with a single instance of this item (for question-
/// type items) or multiple repetitions of the item (for group-type items).
pub repeats: Boolean,
/// An indication, if true, that the item must be present in a "completed"
/// QuestionnaireResponse.  If false, the item may be skipped when answering the
/// questionnaire.
pub required: Boolean,
/// The name of a section, the text of a question or text content for a display
/// item.
pub text: String,
/// The type of questionnaire item this is - whether text for display, a grouping
/// of other items or a particular type of data to be captured (string, integer,
/// Coding, etc.).
pub r#type: Code,
}
