/// A specific set of Roles/Locations/specialties/services that a practitioner may
/// perform at an organization for a period of time.
#[derive(Debug, Clone, PartialEq)]
pub struct PractitionerRole {
    ///  Whether this practitioner role record is in active use. Some systems may use
    /// this property to mark non-active practitioners, such as those that are not
    /// currently employed.
    pub active: super::boolean::Boolean,
    /// A collection of times the practitioner is available or performing this role at
    /// the location and/or healthcareservice.
    pub availability: Vec<super::availability::Availability>,
    /// Collection of characteristics (attributes).
    pub characteristic: Vec<super::codeable_concept::CodeableConcept>,
    /// Roles which this practitioner is authorized to perform for the organization.
    pub code: Vec<super::codeable_concept::CodeableConcept>,
    /// A language the practitioner can use in patient communication. The practitioner
    /// may know several languages (listed in practitioner.communication), however
    /// these are the languages that could be advertised in a directory for a patient
    /// to search.
    pub communication: Vec<super::codeable_concept::CodeableConcept>,
    /// The contact details of communication devices available relevant to the specific
    /// PractitionerRole. This can include addresses, phone numbers, fax numbers, mobile
    /// numbers, email addresses and web sites.
    pub contact: Vec<super::extended_contact_detail::ExtendedContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    ///  Technical endpoints providing access to services operated for the practitioner
    /// with this role. Commonly used for locating scheduling services, or identifying
    /// where to send referrals electronically.
    pub endpoint: Vec<super::reference::Reference>,
    /// The list of healthcare services that this worker provides for this role's
    /// Organization/Location(s).
    pub healthcare_service: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business Identifiers that are specific to a role/location.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The location(s) at which this practitioner provides care.
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
    /// The organization where the Practitioner performs the roles associated.
    pub organization: super::reference::Reference,
    /// The period during which the person is authorized to act as a practitioner in
    /// these role(s) for the organization.
    pub period: super::period::Period,
    /// Practitioner that is able to provide the defined services for the organization.
    pub practitioner: super::reference::Reference,
    /// This is a PractitionerRole resource
    pub resource_type: String,
    /// The specialty of a practitioner that describes the functional role they are
    /// practicing at a given organization or location.
    pub specialty: Vec<super::codeable_concept::CodeableConcept>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}
