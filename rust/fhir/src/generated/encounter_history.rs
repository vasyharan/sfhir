/// A record of significant events/milestones key data throughout the history of an
/// Encounter, often tracked for specific purposes such as billing.
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterHistory {
    /// The start and end time associated with this set of values associated with the
    /// encounter, may be different to the planned times for various reasons.
    pub actual_period: super::period::Period,
    /// Concepts representing classification of patient encounter such as ambulatory
    /// (outpatient), inpatient, emergency, home health or others due to local
    /// variations.
    pub class: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The Encounter associated with this set of historic values.
    pub encounter: super::reference::Reference,
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
    /// The location of the patient at this point in the encounter, the multiple
    /// cardinality permits de-normalizing the levels of the location hierarchy, such as
    /// site/ward/room/bed.
    pub location: Vec<super::encounter_history::EncounterHistoryLocation>,
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
    /// The planned end date/time (or discharge date) of the encounter.
    pub planned_end_date: super::date_time::DateTime,
    /// The planned start date/time (or admission date) of the encounter.
    pub planned_start_date: super::date_time::DateTime,
    /// This is a EncounterHistory resource
    pub resource_type: String,
    /// Broad categorization of the service that is to be provided (e.g. cardiology).
    pub service_type: Vec<super::codeable_reference::CodeableReference>,
    /// planned | in-progress | on-hold | discharged | completed | cancelled |
    /// discontinued | entered-in-error | unknown.
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
}

/// A record of significant events/milestones key data throughout the history of an
/// Encounter, often tracked for specific purposes such as billing.
#[derive(Debug, Clone, PartialEq)]
pub struct EncounterHistoryLocation {
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
}
