/// A container for slots of time that may be available for booking appointments.
#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
    /// Whether this schedule record is in active use or should not be used (such as was
    /// entered in error).
    pub active: super::boolean::Boolean,
    /// Slots that reference this schedule resource provide the availability details to
    /// these referenced resource(s).
    pub actor: Vec<super::reference::Reference>,
    /// Comments on the availability to describe any extended information. Such as
    /// custom constraints on the slots that may be associated.
    pub comment: super::markdown::Markdown,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// External Ids for this item.
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
    /// Further description of the schedule as it would be presented to a consumer while
    /// searching.
    pub name: super::string::String,
    /// The period of time that the slots that reference this Schedule resource cover
    /// (even if none exist). These  cover the amount of time that an organization's
    /// planning horizon; the interval for which they are currently accepting
    /// appointments. This does not define a "template" for planning outside these
    /// dates.
    pub planning_horizon: super::period::Period,
    /// This is a Schedule resource
    pub resource_type: String,
    /// A broad categorization of the service that is to be performed during this
    /// appointment.
    pub service_category: Vec<super::codeable_concept::CodeableConcept>,
    /// The specific service that is to be performed during this appointment.
    pub service_type: Vec<super::codeable_reference::CodeableReference>,
    /// The specialty of a practitioner that would be required to perform the service
    /// requested in this appointment.
    pub specialty: Vec<super::codeable_concept::CodeableConcept>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}
