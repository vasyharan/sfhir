/// A guidance response is the formal response to a guidance request, including any
/// output parameters returned by the evaluation, as well as the description of any
/// proposed actions to be taken.
#[derive(Debug, Clone, PartialEq)]
pub struct GuidanceResponse {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// If the evaluation could not be completed due to lack of information, or
    /// additional information would potentially result in a more accurate response,
    /// this element will a description of the data required in order to proceed with
    /// the evaluation. A subsequent request to the service should include this data.
    pub data_requirement: Vec<super::data_requirement::DataRequirement>,
    /// The encounter during which this response was created or to which the creation of
    /// this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// Messages resulting from the evaluation of the artifact or artifacts. As part
    /// of evaluating the request, the engine may produce informational or warning
    /// messages. These messages will be provided by this element.
    pub evaluation_message: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Allows a service to provide  unique, business identifiers for the response.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
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
    /// An identifier, CodeableConcept or canonical reference to the guidance that was
    /// requested.
    pub module_canonical: String,
    /// An identifier, CodeableConcept or canonical reference to the guidance that was
    /// requested.
    pub module_codeable_concept: super::codeable_concept::CodeableConcept,
    /// An identifier, CodeableConcept or canonical reference to the guidance that was
    /// requested.
    pub module_uri: String,
    /// Provides a mechanism to communicate additional information about the response.
    pub note: Vec<super::annotation::Annotation>,
    /// Indicates when the guidance response was processed.
    pub occurrence_date_time: super::date_time::DateTime,
    /// The output parameters of the evaluation, if any. Many modules will result in
    /// the return of specific resources such as procedure or communication requests
    /// that are returned as part of the operation result. However, modules may define
    /// specific outputs that would be returned as the result of the evaluation, and
    /// these would be returned in this element.
    pub output_parameters: super::reference::Reference,
    /// Provides a reference to the device that performed the guidance.
    pub performer: super::reference::Reference,
    /// Describes the reason for the guidance response in coded or textual form, or
    /// Indicates the reason the request was initiated. This is typically provided
    /// as a parameter to the evaluation and echoed by the service, although for some
    /// use cases, such as subscription- or event-based scenarios, it may provide an
    /// indication of the cause for the response.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The identifier of the request associated with this response. If an identifier
    /// was given as part of the request, it will be reproduced here to enable the
    /// requester to more easily identify the response in a multi-request scenario.
    pub request_identifier: super::identifier::Identifier,
    /// This is a GuidanceResponse resource
    pub resource_type: String,
    /// The actions, if any, produced by the evaluation of the artifact.
    pub result: Vec<super::reference::Reference>,
    /// The status of the response. If the evaluation is completed successfully, the
    /// status will indicate success. However, in order to complete the evaluation,
    /// the engine may require more information. In this case, the status will be data-
    /// required, and the response will contain a description of the additional required
    /// information. If the evaluation completed successfully, but the engine determines
    /// that a potentially more accurate response could be provided if more data was
    /// available, the status will be data-requested, and the response will contain a
    /// description of the additional requested information.
    pub status: super::code::Code,
    /// The patient for which the request was processed.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}