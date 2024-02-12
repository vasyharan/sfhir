/// A formally or informally recognized grouping of people or organizations
/// formed for the purpose of achieving some form of collective action.  Includes
/// companies, institutions, corporations, departments, community groups, healthcare
/// practice groups, payer/insurer, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Organization {
    /// Whether the organization's record is still in active use.
    pub active: super::boolean::Boolean,
    /// A list of alternate names that the organization is known as, or was known as in
    /// the past.
    pub alias: Vec<super::string::String>,
    /// The contact details of communication devices available relevant to the specific
    /// Organization. This can include addresses, phone numbers, fax numbers, mobile
    /// numbers, email addresses and web sites.
    pub contact: Vec<super::extended_contact_detail::ExtendedContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Description of the organization, which helps provide additional general context
    /// on the organization to ensure that the correct organization is selected.
    pub description: super::markdown::Markdown,
    /// Technical endpoints providing access to services operated for the organization.
    pub endpoint: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier for the organization that is used to identify the organization across
    /// multiple disparate systems.
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
    /// A name associated with the organization.
    pub name: super::string::String,
    /// The organization of which this organization forms a part.
    pub part_of: super::reference::Reference,
    /// The official certifications, accreditations, training, designations and
    /// licenses that authorize and/or otherwise endorse the provision of care by the
    /// organization.
    ///
    /// For example, an approval to provide a type of services issued by a certifying
    /// body (such as the US Joint Commission) to an organization.
    pub qualification: Vec<super::organization::OrganizationQualification>,
    /// This is a Organization resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The kind(s) of organization that this is.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// A formally or informally recognized grouping of people or organizations
/// formed for the purpose of achieving some form of collective action.  Includes
/// companies, institutions, corporations, departments, community groups, healthcare
/// practice groups, payer/insurer, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct OrganizationQualification {
    /// Coded representation of the qualification.
    pub code: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// An identifier allocated to this qualification for this organization.
    pub identifier: Vec<super::identifier::Identifier>,
    /// Organization that regulates and issues the qualification.
    pub issuer: super::reference::Reference,
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
    /// Period during which the qualification is valid.
    pub period: super::period::Period,
}
