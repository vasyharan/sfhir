/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireResponse {
    /// The individual or device that received the answers to the questions in the
    /// QuestionnaireResponse and recorded them in the system.
    pub author: super::reference::Reference,
    /// The date and/or time that this questionnaire response was last modified by the
    /// user - e.g. changing answers or revising status.
    pub authored: super::date_time::DateTime,
    /// A plan, proposal or order that is fulfilled in whole or in part by this
    /// questionnaire response.  For example, a ServiceRequest seeking an intake
    /// assessment or a decision support recommendation to assess for post-partum
    /// depression.
    pub based_on: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The Encounter during which this questionnaire response was created or to which
    /// the creation of this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this questionnaire response by the performer
    /// and/or other systems.  These identifiers remain constant as the resource is
    /// updated and propagates from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A group or question item from the original questionnaire for which answers are
    /// provided.
    pub item: Vec<super::questionnaire_response::QuestionnaireResponseItem>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
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
    /// A procedure or observation that this questionnaire was performed as part of the
    /// execution of.  For example, the surgery a checklist was executed as part of.
    pub part_of: Vec<super::reference::Reference>,
    /// The Questionnaire that defines and organizes the questions for which answers are
    /// being provided.
    pub questionnaire: super::canonical::Canonical,
    /// This is a QuestionnaireResponse resource
    pub resource_type: String,
    /// The individual or device that answered the questions about the subject.
    pub source: super::reference::Reference,
    /// The current state of the questionnaire response.
    pub status: super::code::Code,
    /// The subject of the questionnaire response.  This could be a patient,
    /// organization, practitioner, device, etc.  This is who/what the answers apply to,
    /// but is not necessarily the source of information.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireResponseAnswer {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Nested groups and/or questions found within this particular answer.
    pub item: Vec<super::questionnaire_response::QuestionnaireResponseItem>,
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
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_attachment: super::attachment::Attachment,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_boolean: bool,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_coding: super::coding::Coding,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_date: String,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_date_time: String,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_decimal: f64,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_integer: i64,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_quantity: super::quantity::Quantity,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_reference: super::reference::Reference,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_string: String,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_time: String,
    /// The answer (or one of the answers) provided by the respondent to the question.
    pub value_uri: String,
}

/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireResponseItem {
    /// The respondent's answer(s) to the question.
    pub answer: Vec<super::questionnaire_response::QuestionnaireResponseAnswer>,
    /// A reference to an [ElementDefinition](elementdefinition.html) that provides the
    /// details for the item.
    pub definition: super::uri::Uri,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Sub-questions, sub-groups or display items nested beneath a group.
    pub item: Vec<super::questionnaire_response::QuestionnaireResponseItem>,
    /// The item from the Questionnaire that corresponds to this item in the
    /// QuestionnaireResponse resource.
    pub link_id: super::string::String,
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
    /// Text that is displayed above the contents of the group or as the text of the
    /// question being answered.
    pub text: super::string::String,
}
