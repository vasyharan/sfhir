/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.
#[derive(Debug, Clone, PartialEq)]
pub struct RiskAssessment {
    /// A reference to the request that is fulfilled by this risk assessment.
    pub based_on: super::reference::Reference,
    /// Indicates the source data considered as part of the assessment (for example,
    /// FamilyHistory, Observations, Procedures, Conditions, etc.).
    pub basis: Vec<super::reference::Reference>,
    /// The type of the risk assessment performed.
    pub code: super::codeable_concept::CodeableConcept,
    /// For assessments or prognosis specific to a particular condition, indicates the
    /// condition being assessed.
    pub condition: super::reference::Reference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The encounter where the assessment was performed.
    pub encounter: super::reference::Reference,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Business identifier assigned to the risk assessment.
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
    /// The algorithm, process or mechanism used to evaluate the risk.
    pub method: super::codeable_concept::CodeableConcept,
    /// A description of the steps that might be taken to reduce the identified risk(s).
    pub mitigation: super::string::String,
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
    /// Additional comments about the risk assessment.
    pub note: Vec<super::annotation::Annotation>,
    /// The date (and possibly time) the risk assessment was performed.
    pub occurrence_date_time: String,
    /// The date (and possibly time) the risk assessment was performed.
    pub occurrence_period: super::period::Period,
    /// A reference to a resource that this risk assessment is part of, such as a
    /// Procedure.
    pub parent: super::reference::Reference,
    /// The provider, patient, related person, or software application that performed
    /// the assessment.
    pub performer: super::reference::Reference,
    /// Describes the expected outcome for the subject.
    pub prediction: Vec<super::risk_assessment::RiskAssessmentPrediction>,
    /// The reason the risk assessment was performed.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// This is a RiskAssessment resource
    pub resource_type: String,
    /// The status of the RiskAssessment, using the same statuses as an Observation.
    pub status: super::code::Code,
    /// The patient or group the risk assessment applies to.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.
#[derive(Debug, Clone, PartialEq)]
pub struct RiskAssessmentPrediction {
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
    /// One of the potential outcomes for the patient (e.g. remission, death,  a
    /// particular condition).
    pub outcome: super::codeable_concept::CodeableConcept,
    /// Indicates how likely the outcome is (in the specified timeframe).
    pub probability_decimal: f64,
    /// Indicates how likely the outcome is (in the specified timeframe).
    pub probability_range: super::range::Range,
    /// Indicates how likely the outcome is (in the specified timeframe), expressed as a
    /// qualitative value (e.g. low, medium, or high).
    pub qualitative_risk: super::codeable_concept::CodeableConcept,
    /// Additional information explaining the basis for the prediction.
    pub rationale: super::string::String,
    /// Indicates the risk for this particular subject (with their specific
    /// characteristics) divided by the risk of the population in general.  (Numbers
    /// greater than 1 = higher risk than the population, numbers less than 1 = lower
    /// risk.).
    pub relative_risk: super::decimal::Decimal,
    /// Indicates the period of time or age range of the subject to which the specified
    /// probability applies.
    pub when_period: super::period::Period,
    /// Indicates the period of time or age range of the subject to which the specified
    /// probability applies.
    pub when_range: super::range::Range,
}
