/// A patient's point-in-time set of recommendations (i.e. forecasting) according to
/// a published schedule with optional supporting justification.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendation {
    /// Indicates the authority who published the protocol (e.g. ACIP).
    pub authority: super::reference::Reference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date the immunization recommendation(s) were created.
    pub date: super::date_time::DateTime,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this particular recommendation record.
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
    /// The patient the recommendation(s) are for.
    pub patient: super::reference::Reference,
    /// Vaccine administration recommendations.
    pub recommendation:
        Vec<super::immunization_recommendation::ImmunizationRecommendationRecommendation>,
    /// This is a ImmunizationRecommendation resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A patient's point-in-time set of recommendations (i.e. forecasting) according to
/// a published schedule with optional supporting justification.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendationDateCriterion {
    /// Date classification of recommendation.  For example, earliest date to give,
    /// latest date to give, etc.
    pub code: super::codeable_concept::CodeableConcept,
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
    /// The date whose meaning is specified by dateCriterion.code.
    pub value: super::date_time::DateTime,
}

/// A patient's point-in-time set of recommendations (i.e. forecasting) according to
/// a published schedule with optional supporting justification.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationRecommendationRecommendation {
    /// Vaccine(s) which should not be used to fulfill the recommendation.
    pub contraindicated_vaccine_code: Vec<super::codeable_concept::CodeableConcept>,
    /// Vaccine date recommendations.  For example, earliest date to administer, latest
    /// date to administer, etc.
    pub date_criterion:
        Vec<super::immunization_recommendation::ImmunizationRecommendationDateCriterion>,
    /// Contains the description about the protocol under which the vaccine was
    /// administered.
    pub description: super::markdown::Markdown,
    /// Nominal position of the recommended dose in a series as determined by the
    /// evaluation and forecasting process (e.g. dose 2 is the next recommended dose).
    pub dose_number: super::string::String,
    /// The reason for the assigned forecast status.
    pub forecast_reason: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates the patient status with respect to the path to immunity for the target
    /// disease.
    pub forecast_status: super::codeable_concept::CodeableConcept,
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
    /// One possible path to achieve presumed immunity against a disease - within the
    /// context of an authority.
    pub series: super::string::String,
    /// The recommended number of doses to achieve immunity as determined by the
    /// evaluation and forecasting process.
    pub series_doses: super::string::String,
    /// Immunization event history and/or evaluation that supports the status and
    /// recommendation.
    pub supporting_immunization: Vec<super::reference::Reference>,
    /// Patient Information that supports the status and recommendation.  This includes
    /// patient observations, adverse reactions and allergy/intolerance information.
    pub supporting_patient_information: Vec<super::reference::Reference>,
    /// The targeted disease for the recommendation.
    pub target_disease: Vec<super::codeable_concept::CodeableConcept>,
    /// Vaccine(s) or vaccine group that pertain to the recommendation.
    pub vaccine_code: Vec<super::codeable_concept::CodeableConcept>,
}
