/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeader {
    /// The logical author of the message - the personor device that decided the
    /// described event should happen. When there is more than one candidate, pick the
    /// most proximal to the MessageHeader. Can provide other authors in extensions.
    pub author: super::reference::Reference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Permanent link to the MessageDefinition for this message.
    pub definition: super::canonical::Canonical,
    /// The destination application which the message is intended for.
    pub destination: Vec<super::message_header::MessageHeaderDestination>,
    /// Code that identifies the event this message represents and connects it with its
    /// definition. Events defined as part of the FHIR specification are defined by the
    /// implementation.  Alternatively a canonical uri to the EventDefinition.
    pub event_canonical: String,
    /// Code that identifies the event this message represents and connects it with its
    /// definition. Events defined as part of the FHIR specification are defined by the
    /// implementation.  Alternatively a canonical uri to the EventDefinition.
    pub event_coding: super::coding::Coding,
    /// The actual data of the message - a reference to the root/focus class of the
    /// event. This is allowed to be a Parameters resource.
    pub focus: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
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
    /// Coded indication of the cause for the event - indicates  a reason for the
    /// occurrence of the event that is a focus of this message.
    pub reason: super::codeable_concept::CodeableConcept,
    /// This is a MessageHeader resource
    pub resource_type: String,
    /// Information about the message that this message is a response to.  Only present
    /// if this message is a response.
    pub response: super::message_header::MessageHeaderResponse,
    /// The person or organization that accepts overall responsibility for the contents
    /// of the message. The implication is that the message event happened under the
    /// policies of the responsible party.
    pub responsible: super::reference::Reference,
    /// Identifies the sending system to allow the use of a trust relationship.
    pub sender: super::reference::Reference,
    /// The source application from which this message originated.
    pub source: super::message_header::MessageHeaderSource,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeaderDestination {
    /// Indicates where the message should be routed.
    pub endpoint_reference: super::reference::Reference,
    /// Indicates where the message should be routed.
    pub endpoint_url: String,
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
    /// Human-readable name for the target system.
    pub name: super::string::String,
    /// Allows data conveyed by a message to be addressed to a particular person or
    /// department when routing to a specific application isn't sufficient.
    pub receiver: super::reference::Reference,
    /// Identifies the target end system in situations where the initial message
    /// transmission is to an intermediary system.
    pub target: super::reference::Reference,
}

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeaderResponse {
    /// Code that identifies the type of response to the message - whether it was
    /// successful or not, and whether it should be resent or not.
    pub code: super::code::Code,
    /// Full details of any issues found in the message.
    pub details: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The Bundle.identifier of the message to which this message is a response.
    pub identifier: super::identifier::Identifier,
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

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageHeaderSource {
    /// An e-mail, phone, website or other contact point to use to resolve issues with
    /// message communications.
    pub contact: super::contact_point::ContactPoint,
    /// Identifies the routing target to send acknowledgements to.
    pub endpoint_reference: super::reference::Reference,
    /// Identifies the routing target to send acknowledgements to.
    pub endpoint_url: String,
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
    /// Human-readable name for the source system.
    pub name: super::string::String,
    /// May include configuration or other information useful in debugging.
    pub software: super::string::String,
    /// Can convey versions of multiple systems in situations where a message passes
    /// through multiple hands.
    pub version: super::string::String,
}
