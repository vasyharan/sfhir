/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient; e.g. Drug-drug interaction,
/// Ineffective treatment frequency, Procedure-condition conflict, gaps in care,
/// etc.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedIssue {
    /// Individual or device responsible for the issue being raised.  For example, a
    /// decision support application or a pharmacist conducting a medication review.
    pub author: super::reference::Reference,
    /// A code that classifies the general type of detected issue.
    pub category: Vec<super::codeable_concept::CodeableConcept>,
    /// Identifies the specific type of issue identified.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A textual explanation of the detected issue.
    pub detail: super::markdown::Markdown,
    /// The encounter during which this issue was detected.
    pub encounter: super::reference::Reference,
    /// Supporting evidence or manifestations that provide the basis for identifying the
    /// detected issue such as a GuidanceResponse or MeasureReport.
    pub evidence: Vec<super::detected_issue::DetectedIssueEvidence>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// The date or period when the detected issue was initially identified.
    pub identified_date_time: String,
    /// The date or period when the detected issue was initially identified.
    pub identified_period: super::period::Period,
    /// Business identifier associated with the detected issue record.
    pub identifier: Vec<super::identifier::Identifier>,
    /// Indicates the resource representing the current activity or proposed activity
    /// that is potentially problematic.
    pub implicated: Vec<super::reference::Reference>,
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
    /// Indicates an action that has been taken or is committed to reduce or eliminate
    /// the likelihood of the risk identified by the detected issue from manifesting.
    /// Can also reflect an observation of known mitigating factors that may reduce/
    /// eliminate the need for any action.
    pub mitigation: Vec<super::detected_issue::DetectedIssueMitigation>,
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
    /// The literature, knowledge-base or similar reference that describes the
    /// propensity for the detected issue identified.
    pub reference: super::uri::Uri,
    /// This is a DetectedIssue resource
    pub resource_type: String,
    /// Indicates the degree of importance associated with the identified issue based on
    /// the potential impact on the patient.
    pub severity: super::code::Code,
    /// Indicates the status of the detected issue.
    pub status: super::code::Code,
    /// Indicates the subject whose record the detected issue is associated with.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient; e.g. Drug-drug interaction,
/// Ineffective treatment frequency, Procedure-condition conflict, gaps in care,
/// etc.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedIssueEvidence {
    /// A manifestation that led to the recording of this detected issue.
    pub code: Vec<super::codeable_concept::CodeableConcept>,
    /// Links to resources that constitute evidence for the detected issue such as a
    /// GuidanceResponse or MeasureReport.
    pub detail: Vec<super::reference::Reference>,
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

/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient; e.g. Drug-drug interaction,
/// Ineffective treatment frequency, Procedure-condition conflict, gaps in care,
/// etc.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedIssueMitigation {
    /// Describes the action that was taken or the observation that was made that
    /// reduces/eliminates the risk associated with the identified issue.
    pub action: super::codeable_concept::CodeableConcept,
    /// Identifies the practitioner who determined the mitigation and takes
    /// responsibility for the mitigation step occurring.
    pub author: super::reference::Reference,
    /// Indicates when the mitigating action was documented.
    pub date: super::date_time::DateTime,
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
    /// Clinicians may add additional notes or justifications about the mitigation
    /// action. For example, patient can have this drug because they have had it before
    /// without any issues. Multiple justifications may be provided.
    pub note: Vec<super::annotation::Annotation>,
}