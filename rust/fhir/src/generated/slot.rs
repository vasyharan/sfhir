/// A slot of time on a schedule that may be available for booking appointments.
#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    /// The style of appointment or patient that may be booked in the slot (not service
    /// type).
    pub appointment_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Comments on the slot to describe any extended information. Such as custom
    /// constraints on the slot.
    pub comment: super::string::String,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Date/Time that the slot is to conclude.
    pub end: super::instant::Instant,
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
    /// This slot has already been overbooked, appointments are unlikely to be accepted
    /// for this time.
    pub overbooked: super::boolean::Boolean,
    /// This is a Slot resource
    pub resource_type: String,
    /// The schedule resource that this slot defines an interval of status information.
    pub schedule: super::reference::Reference,
    /// A broad categorization of the service that is to be performed during this
    /// appointment.
    pub service_category: Vec<super::codeable_concept::CodeableConcept>,
    /// The type of appointments that can be booked into this slot (ideally this would
    /// be an identifiable service - which is at a location, rather than the location
    /// itself). If provided then this overrides the value provided on the Schedule
    /// resource.
    pub service_type: Vec<super::codeable_reference::CodeableReference>,
    /// The specialty of a practitioner that would be required to perform the service
    /// requested in this appointment.
    pub specialty: Vec<super::codeable_concept::CodeableConcept>,
    /// Date/Time that the slot is to begin.
    pub start: super::instant::Instant,
    /// busy | free | busy-unavailable | busy-tentative | entered-in-error.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}
