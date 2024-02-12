/// The details of a healthcare service available at a location.
#[derive(Debug, Clone, PartialEq)]
pub struct HealthcareService {
    /// This flag is used to mark the record to not be used. This is not used when a
    /// center is closed for maintenance, or for holidays, the notAvailable period is to
    /// be used for this.
    pub active: super::boolean::Boolean,
    /// Indicates whether or not a prospective consumer will require an appointment for
    /// a particular service at a site to be provided by the Organization. Indicates if
    /// an appointment is required for access to this service.
    pub appointment_required: super::boolean::Boolean,
    /// A collection of times that the healthcare service is available.
    pub availability: Vec<super::availability::Availability>,
    /// Identifies the broad category of service being performed or delivered.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Collection of characteristics (attributes).
    pub characteristic: Vec<super::codeable_concept::CodeableConcept>,
    /// Any additional description of the service and/or any specific issues not covered
    /// by the other attributes, which can be displayed as further detail under the
    /// serviceName.
    pub comment: super::markdown::Markdown,
    /// Some services are specifically made available in multiple languages, this
    /// property permits a directory to declare the languages this is offered in.
    /// Typically this is only provided where a service operates in communities with
    /// mixed languages used.
    pub communication: Vec<super::codeable_concept::CodeableConcept>,
    /// The contact details of communication devices available relevant to the specific
    /// HealthcareService. This can include addresses, phone numbers, fax numbers,
    /// mobile numbers, email addresses and web sites.
    pub contact: Vec<super::extended_contact_detail::ExtendedContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The location(s) that this service is available to (not where the service is
    /// provided).
    pub coverage_area: Vec<super::reference::Reference>,
    /// Does this service have specific eligibility requirements that need to be met in
    /// order to use the service?
    pub eligibility: Vec<super::healthcare_service::HealthcareServiceEligibility>,
    /// Technical endpoints providing access to services operated for the specific
    /// healthcare services defined at this resource.
    pub endpoint: Vec<super::reference::Reference>,
    /// Extra details about the service that can't be placed in the other fields.
    pub extra_details: super::markdown::Markdown,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// External identifiers for this item.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The location(s) where this healthcare service may be provided.
    pub location: Vec<super::reference::Reference>,
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
    /// Further description of the service as it would be presented to a consumer while
    /// searching.
    pub name: super::string::String,
    /// When the HealthcareService is representing a specific, schedulable service, the
    /// availableIn property can refer to a generic service.
    pub offered_in: Vec<super::reference::Reference>,
    /// If there is a photo/symbol associated with this HealthcareService, it may be
    /// included here to facilitate quick identification of the service in a list.
    pub photo: super::attachment::Attachment,
    /// Programs that this service is applicable to.
    pub program: Vec<super::codeable_concept::CodeableConcept>,
    /// The organization that provides this healthcare service.
    pub provided_by: super::reference::Reference,
    /// Ways that the service accepts referrals, if this is not provided then it is
    /// implied that no referral is required.
    pub referral_method: Vec<super::codeable_concept::CodeableConcept>,
    /// This is a HealthcareService resource
    pub resource_type: String,
    /// The code(s) that detail the conditions under which the healthcare service is
    /// available/offered.
    pub service_provision_code: Vec<super::codeable_concept::CodeableConcept>,
    /// Collection of specialties handled by the Healthcare service. This is more of a
    /// medical term.
    pub specialty: Vec<super::codeable_concept::CodeableConcept>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The specific type of service that may be delivered or performed.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// The details of a healthcare service available at a location.
#[derive(Debug, Clone, PartialEq)]
pub struct HealthcareServiceEligibility {
    /// Coded value for the eligibility.
    pub code: super::codeable_concept::CodeableConcept,
    /// Describes the eligibility conditions for the service.
    pub comment: super::markdown::Markdown,
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
