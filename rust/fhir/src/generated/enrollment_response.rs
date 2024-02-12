/// This resource provides enrollment and plan details from the processing of an
/// EnrollmentRequest resource.
#[derive(Debug, Clone, PartialEq)]
pub struct EnrollmentResponse {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date when the enclosed suite of services were performed or completed.
    pub created: super::date_time::DateTime,
    /// A description of the status of the adjudication.
    pub disposition: super::string::String,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// The Response business identifier.
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
    /// The Insurer who produced this adjudicated response.
    pub organization: super::reference::Reference,
    /// Processing status: error, complete.
    pub outcome: super::code::Code,
    /// Original request resource reference.
    pub request: super::reference::Reference,
    /// The practitioner who is responsible for the services rendered to the patient.
    pub request_provider: super::reference::Reference,
    /// This is a EnrollmentResponse resource
    pub resource_type: String,
    /// The status of the resource instance.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}
