/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management strategies
/// that are best to manage a patient's condition. Assessments are often 1:1 with
/// a clinical consultation / encounter,  but this varies greatly depending on the
/// clinical workflow. This resource is called "ClinicalImpression" rather than
/// "ClinicalAssessment" to avoid confusion with the recording of assessment tools
/// such as Apgar score.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalImpression {
    /// Change in the status/pattern of a subject's condition since previously assessed,
    /// such as worsening, improving, or no change.  It is a subjective assessment of
    /// the direction of the change.
    pub change_pattern: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Indicates when the documentation of the assessment was complete.
    pub date: super::date_time::DateTime,
    /// A summary of the context and/or cause of the assessment - why / where it was
    /// performed, and what patient events/status prompted it.
    pub description: super::string::String,
    /// The point in time or period over which the subject was assessed.
    pub effective_date_time: String,
    /// The point in time or period over which the subject was assessed.
    pub effective_period: super::period::Period,
    /// The Encounter during which this ClinicalImpression was created or to which the
    /// creation of this record is tightly associated.
    pub encounter: super::reference::Reference,
    /// Specific findings or diagnoses that were considered likely or relevant to
    /// ongoing treatment.
    pub finding: Vec<super::clinical_impression::ClinicalImpressionFinding>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifiers assigned to this clinical impression by the performer or
    /// other systems which remain constant as the resource is updated and propagates
    /// from server to server.
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
    /// Commentary about the impression, typically recorded after the impression itself
    /// was made, though supplemental notes by the original author could also appear.
    pub note: Vec<super::annotation::Annotation>,
    /// The clinician performing the assessment.
    pub performer: super::reference::Reference,
    /// A reference to the last assessment that was conducted on this patient.
    /// Assessments are often/usually ongoing in nature; a care provider (practitioner
    /// or team) will make new assessments on an ongoing basis as new data arises or the
    /// patient's conditions changes.
    pub previous: super::reference::Reference,
    /// A list of the relevant problems/conditions for a patient.
    pub problem: Vec<super::reference::Reference>,
    /// Estimate of likely outcome.
    pub prognosis_codeable_concept: Vec<super::codeable_concept::CodeableConcept>,
    /// RiskAssessment expressing likely outcome.
    pub prognosis_reference: Vec<super::reference::Reference>,
    /// Reference to a specific published clinical protocol that was followed during
    /// this assessment, and/or that provides evidence in support of the diagnosis.
    pub protocol: Vec<super::uri::Uri>,
    /// This is a ClinicalImpression resource
    pub resource_type: String,
    /// Identifies the workflow status of the assessment.
    pub status: super::code::Code,
    /// Captures the reason for the current state of the ClinicalImpression.
    pub status_reason: super::codeable_concept::CodeableConcept,
    /// The patient or group of individuals assessed as part of this record.
    pub subject: super::reference::Reference,
    /// A text summary of the investigations and the diagnosis.
    pub summary: super::string::String,
    /// Information supporting the clinical impression, which can contain investigation
    /// results.
    pub supporting_info: Vec<super::reference::Reference>,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management strategies
/// that are best to manage a patient's condition. Assessments are often 1:1 with
/// a clinical consultation / encounter,  but this varies greatly depending on the
/// clinical workflow. This resource is called "ClinicalImpression" rather than
/// "ClinicalAssessment" to avoid confusion with the recording of assessment tools
/// such as Apgar score.
#[derive(Debug, Clone, PartialEq)]
pub struct ClinicalImpressionFinding {
    /// Which investigations support finding or diagnosis.
    pub basis: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Specific text, code or reference for finding or diagnosis, which may include
    /// ruled-out or resolved conditions.
    pub item: super::codeable_reference::CodeableReference,
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
