/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Clone, PartialEq)]
pub struct Questionnaire {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the {{title}}.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// An identifier for this collection of questions in a particular terminology such
    /// as LOINC.
    pub code: Vec<super::coding::Coding>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the questionnaire and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the questionnaire.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the questionnaire was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the questionnaire changes.
    pub date: super::date_time::DateTime,
    /// The URL of a Questionnaire that this Questionnaire is based on.
    pub derived_from: Vec<super::canonical::Canonical>,
    /// A free text natural language description of the questionnaire from a consumer's
    /// perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the {{title}}.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the questionnaire content was or is planned to be in
    /// active use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the {{title}} for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this questionnaire is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended for genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this questionnaire when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A particular question, question grouping or display text that is part of the
    /// questionnaire.
    pub item: Vec<super::questionnaire::QuestionnaireItem>,
    /// A legal or geographic region in which the questionnaire is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub last_review_date: super::date::Date,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// May be used to represent additional information that is not part of the
    /// basic definition of the resource and that modifies the understanding of the
    /// element that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification.
    /// To make the use of extensions safe and managable, there is a strict set
    /// of governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.
    ///
    /// Modifier extensions SHALL NOT change the meaning of any elements on Resource
    /// or DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
    pub modifier_extension: Vec<super::extension::Extension>,
    /// A natural language name identifying the questionnaire. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the questionnaire.
    pub publisher: super::string::String,
    /// Explanation of why this questionnaire is needed and why it has been designed as
    /// it has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, dependencies,
    /// bibliographic references, and predecessor and successor artifacts.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a Questionnaire resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the {{title}}.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The current state of this questionnaire.
    pub status: super::code::Code,
    /// The types of subjects that can be the subject of responses created for the
    /// questionnaire.
    pub subject_type: Vec<super::code::Code>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the questionnaire.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this questionnaire when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this questionnaire is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the questionnaire is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate questionnaires.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the questionnaire
    /// when it is referenced in a specification, model, design or instance. This is
    /// an arbitrary value managed by the questionnaire author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireAnswerOption {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Indicates whether the answer value is selected when the list of possible answers
    /// is initially shown.
    pub initial_selected: super::boolean::Boolean,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// A potential answer that's allowed as the answer to this question.
    pub value_coding: super::coding::Coding,
    /// A potential answer that's allowed as the answer to this question.
    pub value_date: String,
    /// A potential answer that's allowed as the answer to this question.
    pub value_integer: i64,
    /// A potential answer that's allowed as the answer to this question.
    pub value_reference: super::reference::Reference,
    /// A potential answer that's allowed as the answer to this question.
    pub value_string: String,
    /// A potential answer that's allowed as the answer to this question.
    pub value_time: String,
}

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Clone, PartialEq)]
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
    pub answer_coding: super::coding::Coding,
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
    pub answer_quantity: super::quantity::Quantity,
    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.  If there are multiple answers, a match on any
    /// of the answers suffices.  If different behavior is desired (all must match, at
    /// least 2 must match, etc.), consider using the enableWhenExpression extension.
    pub answer_reference: super::reference::Reference,
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
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// Specifies the criteria by which the question is enabled.
    pub operator: super::code::Code,
    /// The linkId for the question whose answer (or lack of answer) governs whether
    /// this item is enabled.
    pub question: super::string::String,
}

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireInitial {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// The actual value to for an initial answer.
    pub value_attachment: super::attachment::Attachment,
    /// The actual value to for an initial answer.
    pub value_boolean: bool,
    /// The actual value to for an initial answer.
    pub value_coding: super::coding::Coding,
    /// The actual value to for an initial answer.
    pub value_date: String,
    /// The actual value to for an initial answer.
    pub value_date_time: String,
    /// The actual value to for an initial answer.
    pub value_decimal: f64,
    /// The actual value to for an initial answer.
    pub value_integer: i64,
    /// The actual value to for an initial answer.
    pub value_quantity: super::quantity::Quantity,
    /// The actual value to for an initial answer.
    pub value_reference: super::reference::Reference,
    /// The actual value to for an initial answer.
    pub value_string: String,
    /// The actual value to for an initial answer.
    pub value_time: String,
    /// The actual value to for an initial answer.
    pub value_uri: String,
}

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItem {
    /// For items that have a defined set of allowed answers (via answerOption or
    /// answerValueSet), indicates whether values *other* than those specified can be
    /// selected.
    pub answer_constraint: super::code::Code,
    /// One of the permitted answers for the question.
    pub answer_option: Vec<super::questionnaire::QuestionnaireAnswerOption>,
    /// A reference to a value set containing a list of values representing permitted
    /// answers for a question.
    pub answer_value_set: super::canonical::Canonical,
    /// A terminology code that corresponds to this group or question (e.g. a code from
    /// LOINC, which defines many questions and answers).
    pub code: Vec<super::coding::Coding>,
    /// This element is a URI that refers to an [ElementDefinition]
    /// (elementdefinition.html) or to an [ObservationDefinition]
    /// (observationdefinition.html) that provides information about this item,
    /// including information that might otherwise be included in the instance of the
    /// Questionnaire resource. A detailed description of the construction of the URI is
    /// shown in [Comments](questionnaire.html#definition), below.
    pub definition: super::uri::Uri,
    /// Indicates if and how items that are disabled (because enableWhen evaluates to
    /// 'false') should be displayed.
    pub disabled_display: super::code::Code,
    /// Controls how multiple enableWhen values are interpreted -  whether all or any
    /// must be true.
    pub enable_behavior: super::code::Code,
    /// A constraint indicating that this item should only be enabled (displayed/allow
    /// answers to be captured) when the specified condition is true.
    pub enable_when: Vec<super::questionnaire::QuestionnaireEnableWhen>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// One or more values that should be pre-populated in the answer when initially
    /// rendering the questionnaire for user input.
    pub initial: Vec<super::questionnaire::QuestionnaireInitial>,
    /// Text, questions and other groups to be nested beneath a question or group.
    pub item: Vec<super::questionnaire::QuestionnaireItem>,
    /// An identifier that is unique within the Questionnaire allowing linkage to the
    /// equivalent item in a QuestionnaireResponse resource.
    pub link_id: super::string::String,
    /// The maximum number of characters that are permitted in the answer to be
    /// considered a "valid" QuestionnaireResponse.
    pub max_length: super::integer::Integer,
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
    pub modifier_extension: Vec<super::extension::Extension>,
    /// A short label for a particular group, question or set of display text within the
    /// questionnaire used for reference by the individual completing the questionnaire.
    pub prefix: super::string::String,
    /// An indication, when true, that the value cannot be changed by a human respondent
    /// to the Questionnaire.
    pub read_only: super::boolean::Boolean,
    /// An indication, if true, that a QuestionnaireResponse for this item may include
    /// multiple answers associated with a single instance of this item (for question-
    /// type items) or multiple repetitions of the item (for group-type items).
    pub repeats: super::boolean::Boolean,
    /// An indication, if true, that the item must be present in a "completed"
    /// QuestionnaireResponse.  If false, the item may be skipped when answering the
    /// questionnaire.
    pub required: super::boolean::Boolean,
    /// The name of a section, the text of a question or text content for a display
    /// item.
    pub text: super::string::String,
    /// The type of questionnaire item this is - whether text for display, a grouping
    /// of other items or a particular type of data to be captured (string, integer,
    /// Coding, etc.).
    pub r#type: super::code::Code,
}
