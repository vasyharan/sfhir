/// A reply to an appointment request for a patient and/or practitioner(s), such as
/// a confirmation or rejection.
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentResponse {
    /// A Person, Location, HealthcareService, or Device that is participating in the
    /// appointment.
    pub actor: super::reference::Reference,
    /// Appointment that this response is replying to.
    pub appointment: super::reference::Reference,
    /// Additional comments about the appointment.
    pub comment: super::markdown::Markdown,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// This may be either the same as the appointment request to confirm the details
    /// of the appointment, or alternately a new time to request a re-negotiation of the
    /// end time.
    pub end: super::instant::Instant,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// This records identifiers associated with this appointment response concern that
    /// are defined by business processes and/ or used to refer to it when a direct URL
    /// reference to the resource itself is not appropriate.
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
    /// The original date within a recurring request. This could be used in place of the
    /// recurrenceId to be more direct (or where the template is provided through the
    /// simple list of dates in `Appointment.occurrenceDate`).
    pub occurrence_date: super::date::Date,
    /// Participation status of the participant. When the status is declined or
    /// tentative if the start/end times are different to the appointment, then these
    /// times should be interpreted as a requested time change. When the status is
    /// accepted, the times can either be the time of the appointment (as a confirmation
    /// of the time) or can be empty.
    pub participant_status: super::code::Code,
    /// Role of participant in the appointment.
    pub participant_type: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates that the response is proposing a different time that was initially
    /// requested.  The new proposed time will be indicated in the start and end
    /// properties.
    pub proposed_new_time: super::boolean::Boolean,
    /// The recurrence ID (sequence number) of the specific appointment when responding
    /// to a recurring request.
    pub recurrence_id: super::positive_int::PositiveInt,
    /// Indicates that this AppointmentResponse applies to all occurrences in a
    /// recurring request.
    pub recurring: super::boolean::Boolean,
    /// This is a AppointmentResponse resource
    pub resource_type: String,
    /// Date/Time that the appointment is to take place, or requested new start time.
    pub start: super::instant::Instant,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}
