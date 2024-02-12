/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug, Clone, PartialEq)]
pub struct Appointment {
    /// The set of accounts that is expected to be used for billing the activities that
    /// result from this Appointment.
    pub account: Vec<super::reference::Reference>,
    /// The style of appointment or patient that has been booked in the slot (not
    /// service type).
    pub appointment_type: super::codeable_concept::CodeableConcept,
    /// The request this appointment is allocated to assess (e.g. incoming referral or
    /// procedure request).
    pub based_on: Vec<super::reference::Reference>,
    /// The date/time describing when the appointment was cancelled.
    pub cancellation_date: super::date_time::DateTime,
    /// The coded reason for the appointment being cancelled. This is often used
    /// in reporting/billing/futher processing to determine if further actions are
    /// required, or specific fees apply.
    pub cancellation_reason: super::codeable_concept::CodeableConcept,
    /// Concepts representing classification of patient encounter such as ambulatory
    /// (outpatient), inpatient, emergency, home health or others due to local
    /// variations.
    pub class: Vec<super::codeable_concept::CodeableConcept>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date that this appointment was initially created. This could be different to
    /// the meta.lastModified value on the initial entry, as this could have been before
    /// the resource was created on the FHIR server, and should remain unchanged over
    /// the lifespan of the appointment.
    pub created: super::date_time::DateTime,
    /// The brief description of the appointment as would be shown on a subject line in
    /// a meeting request, or appointment list. Detailed or expanded information should
    /// be put in the note field.
    pub description: super::string::String,
    /// Date/Time that the appointment is to conclude.
    pub end: super::instant::Instant,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// This records identifiers associated with this appointment concern that are
    /// defined by business processes and/or used to refer to it when a direct URL
    /// reference to the resource itself is not appropriate (e.g. in CDA documents, or
    /// in written / printed documentation).
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
    /// Number of minutes that the appointment is to take. This can be less than the
    /// duration between the start and end times.  For example, where the actual time
    /// of appointment is only an estimate or if a 30 minute appointment is being
    /// requested, but any time would work.  Also, if there is, for example, a planned
    /// 15 minute break in the middle of a long appointment, the duration may be 15
    /// minutes less than the difference between the start and end.
    pub minutes_duration: super::positive_int::PositiveInt,
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
    /// Additional notes/comments about the appointment.
    pub note: Vec<super::annotation::Annotation>,
    /// This appointment varies from the recurring pattern.
    pub occurrence_changed: super::boolean::Boolean,
    /// The originating appointment in a recurring set of related appointments.
    pub originating_appointment: super::reference::Reference,
    /// List of participants involved in the appointment.
    pub participant: Vec<super::appointment::AppointmentParticipant>,
    /// While Appointment.note contains information for internal use,
    /// Appointment.patientInstructions is used to capture patient facing information
    /// about the Appointment (e.g. please bring your referral or fast from 8pm night
    /// before).
    pub patient_instruction: Vec<super::codeable_reference::CodeableReference>,
    /// The previous appointment in a series of related appointments.
    pub previous_appointment: super::reference::Reference,
    /// The priority of the appointment. Can be used to make informed decisions if
    /// needing to re-prioritize appointments. (The iCal Standard specifies 0 as
    /// undefined, 1 as highest, 9 as lowest priority).
    pub priority: super::codeable_concept::CodeableConcept,
    /// The reason that this appointment is being scheduled. This is more clinical
    /// than administrative. This can be coded, or as specified using information from
    /// another resource. When the patient arrives and the encounter begins it may be
    /// used as the admission diagnosis. The indication will typically be a Condition
    /// (with other resources referenced in the evidence.detail), or a Procedure.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// The sequence number that identifies a specific appointment in a recurring
    /// pattern.
    pub recurrence_id: super::positive_int::PositiveInt,
    /// The details of the recurrence pattern or template that is used to generate
    /// recurring appointments.
    pub recurrence_template: Vec<super::appointment::AppointmentRecurrenceTemplate>,
    /// Appointment replaced by this Appointment in cases where there is a cancellation,
    /// the details of the cancellation can be found in the cancellationReason property
    /// (on the referenced resource).
    pub replaces: Vec<super::reference::Reference>,
    /// A set of date ranges (potentially including times) that the appointment is
    /// preferred to be scheduled within.
    ///
    /// The duration (usually in minutes) could also be provided to indicate the length
    /// of the appointment to fill and populate the start/end times for the actual
    /// allocated time. However, in other situations the duration may be calculated by
    /// the scheduling system.
    pub requested_period: Vec<super::period::Period>,
    /// This is a Appointment resource
    pub resource_type: String,
    /// A broad categorization of the service that is to be performed during this
    /// appointment.
    pub service_category: Vec<super::codeable_concept::CodeableConcept>,
    /// The specific service that is to be performed during this appointment.
    pub service_type: Vec<super::codeable_reference::CodeableReference>,
    /// The slots from the participants' schedules that will be filled by the
    /// appointment.
    pub slot: Vec<super::reference::Reference>,
    /// The specialty of a practitioner that would be required to perform the service
    /// requested in this appointment.
    pub specialty: Vec<super::codeable_concept::CodeableConcept>,
    /// Date/Time that the appointment is to take place.
    pub start: super::instant::Instant,
    /// The overall status of the Appointment. Each of the participants has their own
    /// participation status which indicates their involvement in the process, however
    /// this status indicates the shared status.
    pub status: super::code::Code,
    /// The patient or group associated with the appointment, if they are to be present
    /// (usually) then they should also be included in the participant backbone element.
    pub subject: super::reference::Reference,
    /// Additional information to support the appointment provided when making the
    /// appointment.
    pub supporting_information: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Connection details of a virtual service (e.g. conference call).
    pub virtual_service: Vec<super::virtual_service_detail::VirtualServiceDetail>,
}

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentMonthlyTemplate {
    /// Indicates that appointments in the series of recurring appointments should occur
    /// on a specific day of the month.
    pub day_of_month: super::positive_int::PositiveInt,
    /// Indicates which day of the week the recurring appointments should occur each
    /// nth week.
    pub day_of_week: super::coding::Coding,
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
    /// Indicates that recurring appointments should occur every nth month.
    pub month_interval: super::positive_int::PositiveInt,
    /// Indicates which week within a month the appointments in the series of recurring
    /// appointments should occur on.
    pub nth_week_of_month: super::coding::Coding,
}

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentParticipant {
    /// The individual, device, location, or service participating in the appointment.
    pub actor: super::reference::Reference,
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
    /// Participation period of the actor.
    pub period: super::period::Period,
    /// Whether this participant is required to be present at the meeting. If false, the
    /// participant is optional.
    pub required: super::boolean::Boolean,
    /// Participation status of the actor.
    pub status: super::code::Code,
    /// Role of participant in the appointment.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentRecurrenceTemplate {
    /// Any dates, such as holidays, that should be excluded from the recurrence.
    pub excluding_date: Vec<super::date::Date>,
    /// Any dates, such as holidays, that should be excluded from the recurrence.
    pub excluding_recurrence_id: Vec<super::positive_int::PositiveInt>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Recurring appointments will not occur after this date.
    pub last_occurrence_date: super::date::Date,
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
    /// Information about monthly recurring appointments.
    pub monthly_template: super::appointment::AppointmentMonthlyTemplate,
    /// How many appointments are planned in the recurrence.
    pub occurrence_count: super::positive_int::PositiveInt,
    /// The list of specific dates that will have appointments generated.
    pub occurrence_date: Vec<super::date::Date>,
    /// How often the appointment series should recur.
    pub recurrence_type: super::codeable_concept::CodeableConcept,
    /// The timezone of the recurring appointment occurrences.
    pub timezone: super::codeable_concept::CodeableConcept,
    /// Information about weekly recurring appointments.
    pub weekly_template: super::appointment::AppointmentWeeklyTemplate,
    /// Information about yearly recurring appointments.
    pub yearly_template: super::appointment::AppointmentYearlyTemplate,
}

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentWeeklyTemplate {
    /// Indicates that recurring appointments should occur on Fridays.
    pub friday: super::boolean::Boolean,
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
    /// Indicates that recurring appointments should occur on Mondays.
    pub monday: super::boolean::Boolean,
    /// Indicates that recurring appointments should occur on Saturdays.
    pub saturday: super::boolean::Boolean,
    /// Indicates that recurring appointments should occur on Sundays.
    pub sunday: super::boolean::Boolean,
    /// Indicates that recurring appointments should occur on Thursdays.
    pub thursday: super::boolean::Boolean,
    /// Indicates that recurring appointments should occur on Tuesdays.
    pub tuesday: super::boolean::Boolean,
    /// Indicates that recurring appointments should occur on Wednesdays.
    pub wednesday: super::boolean::Boolean,
    /// The interval defines if the recurrence is every nth week. The default is every
    /// week, so it is expected that this value will be 2 or more.
    ///
    /// e.g. For recurring every second week this interval would be 2, or every third
    /// week the interval would be 3.
    pub week_interval: super::positive_int::PositiveInt,
}

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).
#[derive(Debug, Clone, PartialEq)]
pub struct AppointmentYearlyTemplate {
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
    /// Appointment recurs every nth year.
    pub year_interval: super::positive_int::PositiveInt,
}
