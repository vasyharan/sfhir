/// A clinical or business level record of information being transmitted or
/// shared; e.g. an alert that was sent to a responsible provider, a public health
/// agency communication to a provider/reporter in response to a case report for a
/// reportable condition.
#[derive(Debug, Clone, PartialEq)]
pub struct Communication {
    /// Other resources that pertain to this communication and to which this
    /// communication should be associated.
    pub about: Vec<super::reference::Reference>,
    /// An order, proposal or plan fulfilled in whole or in part by this Communication.
    pub based_on: Vec<super::reference::Reference>,
    /// The type of message conveyed such as alert, notification, reminder, instruction,
    /// etc.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The Encounter during which this Communication was created or to which the
    /// creation of this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this communication by the performer or other
    /// systems which remain constant as the resource is updated and propagates from
    /// server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Prior communication that this communication is in response to.
    pub in_response_to: Vec<super::reference::Reference>,
    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this Communication.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this Communication.
    pub instantiates_uri: Vec<super::uri::Uri>,
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
    /// Additional notes or commentary about the communication by the sender, receiver
    /// or other interested parties.
    pub note: Vec<super::annotation::Annotation>,
    /// A larger event (e.g. Communication, Procedure) of which this particular
    /// communication is a component or step.
    pub part_of: Vec<super::reference::Reference>,
    /// Text, attachment(s), or resource(s) that was communicated to the recipient.
    pub payload: Vec<super::communication::CommunicationPayload>,
    /// Characterizes how quickly the planned or in progress communication must be
    /// addressed. Includes concepts such as stat, urgent, routine.
    pub priority: super::code::Code,
    /// The reason or justification for the communication.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The time when this communication arrived at the destination.
    pub received: super::date_time::DateTime,
    /// The entity (e.g. person, organization, clinical information system, care team or
    /// device) which is the target of the communication.
    pub recipient: Vec<super::reference::Reference>,
    /// This is a Communication resource
    pub resource_type: String,
    /// The entity (e.g. person, organization, clinical information system, or device)
    /// which is the source of the communication.
    pub sender: super::reference::Reference,
    /// The time when this communication was sent.
    pub sent: super::date_time::DateTime,
    /// The status of the transmission.
    pub status: super::code::Code,
    /// Captures the reason for the current state of the Communication.
    pub status_reason: super::codeable_concept::CodeableConcept,
    /// The patient or group that was the focus of this communication.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Description of the purpose/content, similar to a subject line in an email.
    pub topic: super::codeable_concept::CodeableConcept,
}

/// A clinical or business level record of information being transmitted or
/// shared; e.g. an alert that was sent to a responsible provider, a public health
/// agency communication to a provider/reporter in response to a case report for a
/// reportable condition.
#[derive(Debug, Clone, PartialEq)]
pub struct CommunicationPayload {
    /// A communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub content_attachment: super::attachment::Attachment,
    /// A communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub content_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A communicated content (or for multi-part communications, one portion of the
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
