/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEvent {
    /// Whether the event actually happened or was a near miss. Note that this is
    /// independent of whether anyone was affected or harmed or how severely.
    pub actuality: super::code::Code,
    /// The overall type of event, intended for search and filtering purposes.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Specific event that occurred or that was averted, such as patient fall, wrong
    /// organ removed, or wrong blood transfused.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The contributing factors suspected to have increased the probability or severity
    /// of the adverse event.
    pub contributing_factor: Vec<super::adverse_event::AdverseEventContributingFactor>,
    /// Estimated or actual date the AdverseEvent began, in the opinion of the reporter.
    pub detected: super::date_time::DateTime,
    /// The Encounter associated with the start of the AdverseEvent.
    pub encounter: super::reference::Reference,
    /// Considered likely or probable or anticipated in the research study.  Whether the
    /// reported event matches any of the outcomes for the patient that are considered
    /// by the study as known or likely.
    pub expected_in_research_study: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this adverse event by the performer or other
    /// systems which remain constant as the resource is updated and propagates from
    /// server to server.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The information about where the adverse event occurred.
    pub location: super::reference::Reference,
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub meta: super::meta::Meta,
    /// The ameliorating action taken after the adverse event occured in order to reduce
    /// the extent of harm.
    pub mitigating_action: Vec<super::adverse_event::AdverseEventMitigatingAction>,
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
    /// Comments made about the adverse event by the performer, subject or other
    /// participants.
    pub note: Vec<super::annotation::Annotation>,
    /// The date (and perhaps time) when the adverse event occurred.
    pub occurrence_date_time: String,
    /// The date (and perhaps time) when the adverse event occurred.
    pub occurrence_period: super::period::Period,
    /// The date (and perhaps time) when the adverse event occurred.
    pub occurrence_timing: super::timing::Timing,
    /// Describes the type of outcome from the adverse event, such as resolved,
    /// recovering, ongoing, resolved-with-sequelae, or fatal.
    pub outcome: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates who or what participated in the adverse event and how they were
    /// involved.
    pub participant: Vec<super::adverse_event::AdverseEventParticipant>,
    /// Preventive actions that contributed to avoiding the adverse event.
    pub preventive_action: Vec<super::adverse_event::AdverseEventPreventiveAction>,
    /// The date on which the existence of the AdverseEvent was first recorded.
    pub recorded_date: super::date_time::DateTime,
    /// Information on who recorded the adverse event.  May be the patient or a
    /// practitioner.
    pub recorder: super::reference::Reference,
    /// This is a AdverseEvent resource
    pub resource_type: String,
    /// Information about the condition that occurred as a result of the adverse event,
    /// such as hives due to the exposure to a substance (for example, a drug or a
    /// chemical) or a broken leg as a result of the fall.
    pub resulting_effect: Vec<super::reference::Reference>,
    /// Assessment whether this event, or averted event, was of clinical importance.
    pub seriousness: super::codeable_concept::CodeableConcept,
    /// The current state of the adverse event or potential adverse event.
    pub status: super::code::Code,
    /// The research study that the subject is enrolled in.
    pub study: Vec<super::reference::Reference>,
    /// This subject or group impacted by the event.
    pub subject: super::reference::Reference,
    /// Supporting information relevant to the event.
    pub supporting_info: Vec<super::adverse_event::AdverseEventSupportingInfo>,
    /// Describes the entity that is suspected to have caused the adverse event.
    pub suspect_entity: Vec<super::adverse_event::AdverseEventSuspectEntity>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventCausality {
    /// The method of evaluating the relatedness of the suspected entity to the event.
    pub assessment_method: super::codeable_concept::CodeableConcept,
    /// The author of the information on the possible cause of the event.
    pub author: super::reference::Reference,
    /// The result of the assessment regarding the relatedness of the suspected entity
    /// to the event.
    pub entity_relatedness: super::codeable_concept::CodeableConcept,
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
}

/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventContributingFactor {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The item that is suspected to have increased the probability or severity of the
    /// adverse event.
    pub item_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The item that is suspected to have increased the probability or severity of the
    /// adverse event.
    pub item_reference: super::reference::Reference,
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

/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventMitigatingAction {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The ameliorating action taken after the adverse event occured in order to reduce
    /// the extent of harm.
    pub item_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The ameliorating action taken after the adverse event occured in order to reduce
    /// the extent of harm.
    pub item_reference: super::reference::Reference,
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

/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventParticipant {
    /// Indicates who or what participated in the event.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of involvement of the actor in the adverse event, such as
    /// contributor or informant.
    pub function: super::codeable_concept::CodeableConcept,
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
}

/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventPreventiveAction {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The action that contributed to avoiding the adverse event.
    pub item_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The action that contributed to avoiding the adverse event.
    pub item_reference: super::reference::Reference,
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

/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventSupportingInfo {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Relevant past history for the subject. In a clinical care context, an example
    /// being a patient had an adverse event following a pencillin administration and
    /// the patient had a previously documented penicillin allergy. In a clinical trials
    /// context, an example is a bunion or rash that was present prior to the study.
    /// Additionally, the supporting item can be a document that is relevant to this
    /// instance of the adverse event that is not part of the subject's medical history.
    /// For example, a clinical note, staff list, or material safety data sheet (MSDS).
    /// Supporting information is not a contributing factor, preventive action, or
    /// mitigating action.
    pub item_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Relevant past history for the subject. In a clinical care context, an example
    /// being a patient had an adverse event following a pencillin administration and
    /// the patient had a previously documented penicillin allergy. In a clinical trials
    /// context, an example is a bunion or rash that was present prior to the study.
    /// Additionally, the supporting item can be a document that is relevant to this
    /// instance of the adverse event that is not part of the subject's medical history.
    /// For example, a clinical note, staff list, or material safety data sheet (MSDS).
    /// Supporting information is not a contributing factor, preventive action, or
    /// mitigating action.
    pub item_reference: super::reference::Reference,
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

/// An event (i.e. any change to current patient status) that may be related
/// to unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major domains
/// where the AdverseEvent resource is expected to be used. One is in clinical care
/// reported adverse events and the other is in reporting adverse events in clinical
/// research trial management. Adverse events can be reported by healthcare
/// providers, patients, caregivers or by medical products manufacturers. Given
/// the differences between these two concepts, we recommend consulting the domain
/// specific implementation guides when implementing the AdverseEvent Resource. The
/// implementation guides include specific extensions, value sets and constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct AdverseEventSuspectEntity {
    /// Information on the possible cause of the event.
    pub causality: super::adverse_event::AdverseEventCausality,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Identifies the actual instance of what caused the adverse event.  May be a
    /// substance, medication, medication administration, medication statement or
    /// a device.
    pub instance_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Identifies the actual instance of what caused the adverse event.  May be a
    /// substance, medication, medication administration, medication statement or
    /// a device.
    pub instance_reference: super::reference::Reference,
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
