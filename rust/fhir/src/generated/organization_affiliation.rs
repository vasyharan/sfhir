/// Defines an affiliation/association/relationship between 2 distinct
/// organizations, that is not a part-of relationship/sub-division relationship.
#[derive(Debug, Clone, PartialEq)]
pub struct OrganizationAffiliation {
    /// Whether this organization affiliation record is in active use.
    pub active: super::boolean::Boolean,
    /// Definition of the role the participatingOrganization plays in the association.
    pub code: Vec<super::codeable_concept::CodeableConcept>,
    /// The contact details of communication devices available at the
    /// participatingOrganization relevant to this Affiliation.
    pub contact: Vec<super::extended_contact_detail::ExtendedContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Technical endpoints providing access to services operated for this role.
    pub endpoint: Vec<super::reference::Reference>,
    /// Healthcare services provided through the role.
    pub healthcare_service: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers that are specific to this role.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The location(s) at which the role occurs.
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
    /// The network in which the participatingOrganization provides the role's services
    /// (if defined) at the indicated locations (if defined).
    pub network: Vec<super::reference::Reference>,
    /// Organization where the role is available (primary organization/has members).
    pub organization: super::reference::Reference,
    /// The Participating Organization provides/performs the role(s) defined by the code
    /// to the Primary Organization (e.g. providing services or is a member of).
    pub participating_organization: super::reference::Reference,
    /// The period during which the participatingOrganization is affiliated with the
    /// primary organization.
    pub period: super::period::Period,
    /// This is a OrganizationAffiliation resource
    pub resource_type: String,
    /// Specific specialty of the participatingOrganization in the context of the role.
    pub specialty: Vec<super::codeable_concept::CodeableConcept>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}