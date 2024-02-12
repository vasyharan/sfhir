/// A request to convey information; e.g. the CDS system proposes that an alert be
/// sent to a responsible provider, the CDS system proposes that the public health
/// agency be notified about a reportable condition.
#[derive(Debug, Clone, PartialEq)]
pub struct CommunicationRequest {
    /// Other resources that pertain to this communication request and to which this
    /// communication request should be associated.
    pub about: Vec<super::reference::Reference>,
    /// For draft requests, indicates the date of initial creation.  For requests with
    /// other statuses, indicates the date of activation.
    pub authored_on: super::date_time::DateTime,
    /// A plan or proposal that is fulfilled in whole or in part by this request.
    pub based_on: Vec<super::reference::Reference>,
    /// The type of message to be sent such as alert, notification, reminder,
    /// instruction, etc.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// If true indicates that the CommunicationRequest is asking for the specified
    /// action to *not* occur.
    pub do_not_perform: super::boolean::Boolean,
    /// The Encounter during which this CommunicationRequest was created or to which the
    /// creation of this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// A shared identifier common to multiple independent Request instances that
    /// were activated/authorized more or less simultaneously by a single author.  The
    /// presence of the same identifier on each request ties those requests together and
    /// may have business ramifications in terms of reporting of results, billing, etc.
    /// E.g. a requisition number shared by a set of lab tests ordered together, or a
    /// prescription number shared by all meds ordered at one time.
    pub group_identifier: super::identifier::Identifier,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this communication request by the performer or
    /// other systems which remain constant as the resource is updated and propagates
    /// from server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The entity (e.g. person, organization, clinical information system, or device)
    /// which is to be the source of the communication.
    pub information_provider: Vec<super::reference::Reference>,
    /// Indicates the level of authority/intentionality associated with the
    /// CommunicationRequest and where the request fits into the workflow chain.
    pub intent: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// A channel that was used for this communication (e.g. email, fax).
    pub medium: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Comments made about the request by the requester, sender, recipient, subject or
    /// other participants.
    pub note: Vec<super::annotation::Annotation>,
    /// The time when this communication is to occur.
    pub occurrence_date_time: String,
    /// The time when this communication is to occur.
    pub occurrence_period: super::period::Period,
    /// Text, attachment(s), or resource(s) to be communicated to the recipient.
    pub payload: Vec<super::communication_request::CommunicationRequestPayload>,
    /// Characterizes how quickly the proposed act must be initiated. Includes concepts
    /// such as stat, urgent, routine.
    pub priority: super::code::Code,
    /// Describes why the request is being made in coded or textual form.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The entity (e.g. person, organization, clinical information system, device,
    /// group, or care team) which is the intended target of the communication.
    pub recipient: Vec<super::reference::Reference>,
    /// Completed or terminated request(s) whose function is taken by this new request.
    pub replaces: Vec<super::reference::Reference>,
    /// The device, individual, or organization who asks for the information to be
    /// shared.
    pub requester: super::reference::Reference,
    /// This is a CommunicationRequest resource
    pub resource_type: String,
    /// The status of the proposal or order.
    pub status: super::code::Code,
    /// Captures the reason for the current state of the CommunicationRequest.
    pub status_reason: super::codeable_concept::CodeableConcept,
    /// The patient or group that is the focus of this communication request.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A request to convey information; e.g. the CDS system proposes that an alert be
/// sent to a responsible provider, the CDS system proposes that the public health
/// agency be notified about a reportable condition.
#[derive(Debug, Clone, PartialEq)]
pub struct CommunicationRequestPayload {
    /// The communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub content_attachment: super::attachment::Attachment,
    /// The communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub content_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub content_reference: super::reference::Reference,
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
}
