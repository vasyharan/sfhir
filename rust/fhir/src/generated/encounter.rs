/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
/// Encounter is primarily used to record information about the actual activities
/// that occurred, where Appointment is used to record planned activities.
#[derive(Debug, Clone, PartialEq)]
pub struct Encounter {
    /// The set of accounts that may be used for billing for this Encounter.
    pub account: Vec<super::reference::Reference>,
    /// The actual start and end time of the encounter.
    pub actual_period: super::period::Period,
    /// Details about the stay during which a healthcare service is provided.
    ///
    /// This does not describe the event of admitting the patient, but rather any
    /// information that is relevant from the time of admittance until the time of
    /// discharge.
    pub admission: super::encounter::EncounterAdmission,
    /// The appointment that scheduled this encounter.
    pub appointment: Vec<super::reference::Reference>,
    /// The request this encounter satisfies (e.g. incoming referral or procedure
    /// request).
    pub based_on: Vec<super::reference::Reference>,
    /// The group(s) of individuals, organizations that are allocated to participate in
    /// this encounter. The participants backbone will record the actuals of when these
    /// individuals participated during the encounter.
    pub care_team: Vec<super::reference::Reference>,
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
    /// The list of diagnosis relevant to this encounter.
    pub diagnosis: Vec<super::encounter::EncounterDiagnosis>,
    /// Diet preferences reported by the patient.
    pub diet_preference: Vec<super::codeable_concept::CodeableConcept>,
    /// Where a specific encounter should be classified as a part of a specific
    /// episode(s) of care this field should be used. This association can facilitate
    /// grouping of related encounters together for a specific purpose, such as
    /// government reporting, issue tracking, association via a common problem.  The
    /// association is recorded on the encounter as these are typically created after
    /// the episode of care and grouped on entry rather than editing the episode of care
    /// to append another encounter to it (the episode of care could span years).
    pub episode_of_care: Vec<super::reference::Reference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifier(s) by which this encounter is known.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// Actual quantity of time the encounter lasted. This excludes the time during
    /// leaves of absence.
    ///
    /// When missing it is the time in between the start and end values.
    pub length: super::duration::Duration,
    /// List of locations where  the patient has been during this encounter.
    pub location: Vec<super::encounter::EncounterLocation>,
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
    /// Another Encounter of which this encounter is a part of (administratively or
    /// in time).
    pub part_of: super::reference::Reference,
    /// The list of people responsible for providing the service.
    pub participant: Vec<super::encounter::EncounterParticipant>,
    /// The planned end date/time (or discharge date) of the encounter.
    pub planned_end_date: super::date_time::DateTime,
    /// The planned start date/time (or admission date) of the encounter.
    pub planned_start_date: super::date_time::DateTime,
    /// Indicates the urgency of the encounter.
    pub priority: super::codeable_concept::CodeableConcept,
    /// The list of medical reasons that are expected to be addressed during the episode
    /// of care.
    pub reason: Vec<super::encounter::EncounterReason>,
    /// This is a Encounter resource
    pub resource_type: String,
    /// The organization that is primarily responsible for this Encounter's services.
    /// This MAY be the same as the organization on the Patient record, however it could
    /// be different, such as if the actor performing the services was from an external
    /// organization (which may be billed seperately) for an external consultation.
    /// Refer to the colonoscopy example on the Encounter examples tab.
    pub service_provider: super::reference::Reference,
    /// Broad categorization of the service that is to be provided (e.g. cardiology).
    pub service_type: Vec<super::codeable_reference::CodeableReference>,
    /// Any special requests that have been made for this encounter, such as the
    /// provision of specific equipment or other things.
    pub special_arrangement: Vec<super::codeable_concept::CodeableConcept>,
    /// Special courtesies that may be provided to the patient during the encounter
    /// (VIP, board member, professional courtesy).
    pub special_courtesy: Vec<super::codeable_concept::CodeableConcept>,
    /// The current state of the encounter (not the state of the patient within the
    /// encounter - that is subjectState).
    pub status: super::code::Code,
    /// The patient or group related to this encounter. In some use-cases the patient
    /// MAY not be present, such as a case meeting about a patient between several
    /// practitioners or a careteam.
    pub subject: super::reference::Reference,
    /// The subjectStatus value can be used to track the patient's status within the
    /// encounter. It details whether the patient has arrived or departed, has been
    /// triaged or is currently in a waiting status.
    pub subject_status: super::codeable_concept::CodeableConcept,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled
    /// nursing, rehabilitation).
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
    /// Connection details of a virtual service (e.g. conference call).
    pub virtual_service: Vec<super::virtual_service_detail::VirtualServiceDetail>,
}

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
/// Encounter is primarily used to record information about the actual activities
/// that occurred, where Appointment is used to record planned activities.
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterAdmission {
    /// From where patient was admitted (physician referral, transfer).
    pub admit_source: super::codeable_concept::CodeableConcept,
    /// Location/organization to which the patient is discharged.
    pub destination: super::reference::Reference,
    /// Category or kind of location after discharge.
    pub discharge_disposition: super::codeable_concept::CodeableConcept,
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
    /// The location/organization from which the patient came before admission.
    pub origin: super::reference::Reference,
    /// Pre-admission identifier.
    pub pre_admission_identifier: super::identifier::Identifier,
    /// Indicates that this encounter is directly related to a prior admission,
    /// often because the conditions addressed in the prior admission were not fully
    /// addressed.
    pub re_admission: super::codeable_concept::CodeableConcept,
}

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
/// Encounter is primarily used to record information about the actual activities
/// that occurred, where Appointment is used to record planned activities.
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterDiagnosis {
    /// The coded diagnosis or a reference to a Condition (with other resources
    /// referenced in the evidence.detail), the use property will indicate the purpose
    /// of this specific diagnosis.
    pub condition: Vec<super::codeable_reference::CodeableReference>,
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
    /// Role that this diagnosis has within the encounter (e.g. admission, billing,
    /// discharge …).
    pub r#use: Vec<super::codeable_concept::CodeableConcept>,
}

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
/// Encounter is primarily used to record information about the actual activities
/// that occurred, where Appointment is used to record planned activities.
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterLocation {
    /// This will be used to specify the required levels (bed/ward/room/etc.) desired to
    /// be recorded to simplify either messaging or query.
    pub form: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The location where the encounter takes place.
    pub location: super::reference::Reference,
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
    /// Time period during which the patient was present at the location.
    pub period: super::period::Period,
    /// The status of the participants' presence at the specified location during the
    /// period specified. If the participant is no longer at the location, then the
    /// period will have an end date/time.
    pub status: super::code::Code,
}

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
/// Encounter is primarily used to record information about the actual activities
/// that occurred, where Appointment is used to record planned activities.
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterParticipant {
    /// Person involved in the encounter, the patient/group is also included here to
    /// indicate that the patient was actually participating in the encounter. Not
    /// including the patient here covers use cases such as a case meeting between
    /// practitioners about a patient - non contact times.
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
    /// The period of time that the specified participant participated in the encounter.
    /// These can overlap or be sub-sets of the overall encounter's period.
    pub period: super::period::Period,
    /// Role of participant in encounter.
    pub r#type: Vec<super::codeable_concept::CodeableConcept>,
}

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
/// Encounter is primarily used to record information about the actual activities
/// that occurred, where Appointment is used to record planned activities.
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterReason {
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
    /// What the reason value should be used as e.g. Chief Complaint, Health Concern,
    /// Health Maintenance (including screening).
    pub r#use: Vec<super::codeable_concept::CodeableConcept>,
    /// Reason the encounter takes place, expressed as a code or a reference to another
    /// resource. For admissions, this can be used for a coded admission diagnosis.
    pub value: Vec<super::codeable_reference::CodeableReference>,
}
