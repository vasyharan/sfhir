/// The technical details of an endpoint that can be used for electronic services,
/// such as for web services providing XDS.b, a REST endpoint for another FHIR
/// server, or a s/Mime email address. This may include any security context
/// information.
#[derive(Debug, Clone, PartialEq)]
pub struct Endpoint {
    /// The uri that describes the actual end-point to connect to.
    pub address: super::url::Url,
    /// A coded value that represents the technical details of the usage of this
    /// endpoint, such as what WSDLs should be used in what way. (e.g. XDS.b/DICOM/
    /// cds-hook).
    pub connection_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Contact details for a human to contact about the endpoint. The primary use of
    /// this for system administrator troubleshooting.
    pub contact: Vec<super::contact_point::ContactPoint>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The description of the endpoint and what it is for (typically used as
    /// supplemental information in an endpoint directory describing its usage/purpose).
    pub description: super::string::String,
    /// The type of environment(s) exposed at this endpoint (dev, prod, test, etc.).
    pub environment_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Additional headers / information to send as part of the notification.
    pub header: Vec<super::string::String>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier for the organization that is used to identify the endpoint across
    /// multiple disparate systems.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The organization that manages this endpoint (even if technically another
    /// organization is hosting this in the cloud, it is the organization associated
    /// with the data).
    pub managing_organization: super::reference::Reference,
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
    /// A friendly name that this endpoint can be referred to with.
    pub name: super::string::String,
    /// The set of payloads that are provided/available at this endpoint.
    pub payload: Vec<super::endpoint::EndpointPayload>,
    /// The interval during which the endpoint is expected to be operational.
    pub period: super::period::Period,
    /// This is a Endpoint resource
    pub resource_type: String,
    /// The endpoint status represents the general expected availability of an endpoint.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// The technical details of an endpoint that can be used for electronic services,
/// such as for web services providing XDS.b, a REST endpoint for another FHIR
/// server, or a s/Mime email address. This may include any security context
/// information.
#[derive(Debug, Clone, PartialEq)]
pub struct EndpointPayload {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The mime type to send the payload in - e.g. application/fhir+xml, application/
    /// fhir+json. If the mime type is not specified, then the sender could send any
    /// content (including no content depending on the connectionType).
    pub mime_type: Vec<super::code::Code>,
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
    /// The payload type describes the acceptable content that can be communicated on
    /// the endpoint.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}
