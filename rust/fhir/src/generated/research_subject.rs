/// A ResearchSubject is a participant or object which is the recipient of
/// investigative activities in a research study.
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchSubject {
    /// The name of the arm in the study the subject actually followed as part of this
    /// study.
    pub actual_comparison_group: super::id::Id,
    /// The name of the arm in the study the subject is expected to follow as part of
    /// this study.
    pub assigned_comparison_group: super::id::Id,
    /// A record of the patient's informed agreement to participate in the study.
    pub consent: Vec<super::reference::Reference>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Identifiers assigned to this research subject for a study.
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
    /// The dates the subject began and ended their participation in the study.
    pub period: super::period::Period,
    /// The current state (status) of the subject and resons for status change where
    /// appropriate.
    pub progress: Vec<super::research_subject::ResearchSubjectProgress>,
    /// This is a ResearchSubject resource
    pub resource_type: String,
    /// The publication state of the resource (not of the subject).
    pub status: super::code::Code,
    /// Reference to the study the subject is participating in.
    pub study: super::reference::Reference,
    /// The record of the person, animal or other entity involved in the study.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A ResearchSubject is a participant or object which is the recipient of
/// investigative activities in a research study.
#[derive(Debug, Clone, PartialEq)]
pub struct ResearchSubjectProgress {
    /// The date when the state ended.
    pub end_date: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The milestones the subject has passed through.
    pub milestone: super::codeable_concept::CodeableConcept,
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
    /// The reason for the state change.  If coded it should follow the formal subject
    /// state model.
    pub reason: super::codeable_concept::CodeableConcept,
    /// The date when the new status started.
    pub start_date: super::date_time::DateTime,
    /// The current state of the subject.
    pub subject_state: super::codeable_concept::CodeableConcept,
    /// Identifies the aspect of the subject's journey that the state refers to.
    pub r#type: super::codeable_concept::CodeableConcept,
}
