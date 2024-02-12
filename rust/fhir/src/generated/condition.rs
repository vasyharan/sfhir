/// A clinical condition, problem, diagnosis, or other event, situation, issue, or
/// clinical concept that has risen to a level of concern.
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Some conditions, such as chronic
    /// conditions, are never really resolved, but they can abate.
    pub abatement_age: super::age::Age,
    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Some conditions, such as chronic
    /// conditions, are never really resolved, but they can abate.
    pub abatement_date_time: String,
    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Some conditions, such as chronic
    /// conditions, are never really resolved, but they can abate.
    pub abatement_period: super::period::Period,
    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Some conditions, such as chronic
    /// conditions, are never really resolved, but they can abate.
    pub abatement_range: super::range::Range,
    /// The date or estimated date that the condition resolved or went into remission.
    /// This is called "abatement" because of the many overloaded connotations
    /// associated with "remission" or "resolution" - Some conditions, such as chronic
    /// conditions, are never really resolved, but they can abate.
    pub abatement_string: String,
    /// The anatomical location where this condition manifests itself.
    pub body_site: Vec<super::codeable_concept::CodeableConcept>,
    /// A category assigned to the condition.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// The clinical status of the condition.
    pub clinical_status: super::codeable_concept::CodeableConcept,
    /// Identification of the condition, problem or diagnosis.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The Encounter during which this Condition was created or to which the creation
    /// of this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// Supporting evidence / manifestations that are the basis of the Condition's
    /// verification status, such as evidence that confirmed or refuted the condition.
    pub evidence: Vec<super::codeable_reference::CodeableReference>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this condition by the performer or other
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
    /// Additional information about the Condition. This is a general notes/comments
    /// entry  for description of the Condition, its diagnosis and prognosis.
    pub note: Vec<super::annotation::Annotation>,
    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub onset_age: super::age::Age,
    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub onset_date_time: String,
    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub onset_period: super::period::Period,
    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub onset_range: super::range::Range,
    /// Estimated or actual date or date-time  the condition began, in the opinion of
    /// the clinician.
    pub onset_string: String,
    /// Indicates who or what participated in the activities related to the condition
    /// and how they were involved.
    pub participant: Vec<super::condition::ConditionParticipant>,
    /// The recordedDate represents when this particular Condition record was created in
    /// the system, which is often a system-generated date.
    pub recorded_date: super::date_time::DateTime,
    /// This is a Condition resource
    pub resource_type: String,
    /// A subjective assessment of the severity of the condition as evaluated by the
    /// clinician.
    pub severity: super::codeable_concept::CodeableConcept,
    /// A simple summary of the stage such as "Stage 3" or "Early Onset". The
    /// determination of the stage is disease-specific, such as cancer, retinopathy of
    /// prematurity, kidney diseases, Alzheimer's, or Parkinson disease.
    pub stage: Vec<super::condition::ConditionStage>,
    /// Indicates the patient or group who the condition record is associated with.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The verification status to support the clinical status of the condition.  The
    /// verification status pertains to the condition, itself, not to any specific
    /// condition attribute.
    pub verification_status: super::codeable_concept::CodeableConcept,
}

/// A clinical condition, problem, diagnosis, or other event, situation, issue, or
/// clinical concept that has risen to a level of concern.
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionParticipant {
    /// Indicates who or what participated in the activities related to the condition.
    pub actor: super::reference::Reference,
    /// Distinguishes the type of involvement of the actor in the activities related to
    /// the condition.
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

/// A clinical condition, problem, diagnosis, or other event, situation, issue, or
/// clinical concept that has risen to a level of concern.
#[derive(Debug, Clone, PartialEq)]
pub struct ConditionStage {
    /// Reference to a formal record of the evidence on which the staging assessment
    /// is based.
    pub assessment: Vec<super::reference::Reference>,
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
    /// A simple summary of the stage such as "Stage 3" or "Early Onset". The
    /// determination of the stage is disease-specific, such as cancer, retinopathy of
    /// prematurity, kidney diseases, Alzheimer's, or Parkinson disease.
    pub summary: super::codeable_concept::CodeableConcept,
    /// The kind of staging, such as pathological or clinical staging.
    pub r#type: super::codeable_concept::CodeableConcept,
}
