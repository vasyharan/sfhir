/// Prospective warnings of potential issues when providing care to the patient.
#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    /// The person, organization or device that created the flag.
    pub author: super::reference::Reference,
    /// Allows a flag to be divided into different categories like clinical,
    /// administrative etc. Intended to be used as a means of filtering which flags are
    /// displayed to particular user or in a given context.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// The coded value or textual component of the flag to display to the user.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// This alert is only relevant during the encounter.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this flag by the performer or other systems
    /// which remain constant as the resource is updated and propagates from server
    /// to server.
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
    /// The period of time from the activation of the flag to inactivation of the flag.
    /// If the flag is active, the end of the period should be unspecified.
    pub period: super::period::Period,
    /// This is a Flag resource
    pub resource_type: String,
    /// Supports basic workflow.
    pub status: super::code::Code,
    /// The patient, related person, location, group, organization, or practitioner etc.
    /// this is about record this flag is associated with.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}
