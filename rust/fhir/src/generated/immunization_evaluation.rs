/// Describes a comparison of an immunization event against published
/// recommendations to determine if the administration is "valid" in relation to
/// those  recommendations.
#[derive(Debug, Clone, PartialEq)]
pub struct ImmunizationEvaluation {
    /// Indicates the authority who published the protocol (e.g. ACIP).
    pub authority: super::reference::Reference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The date the evaluation of the vaccine administration event was performed.
    pub date: super::date_time::DateTime,
    /// Additional information about the evaluation.
    pub description: super::markdown::Markdown,
    /// Nominal position in a series as determined by the outcome of the evaluation
    /// process.
    pub dose_number: super::string::String,
    /// Indicates if the dose is valid or not valid with respect to the published
    /// recommendations.
    pub dose_status: super::codeable_concept::CodeableConcept,
    /// Provides an explanation as to why the vaccine administration event is valid or
    /// not relative to the published recommendations.
    pub dose_status_reason: Vec<super::codeable_concept::CodeableConcept>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A unique identifier assigned to this immunization evaluation record.
    pub identifier: Vec<super::identifier::Identifier>,
    /// The vaccine administration event being evaluated.
    pub immunization_event: super::reference::Reference,
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
    /// The individual for whom the evaluation is being done.
    pub patient: super::reference::Reference,
    /// This is a ImmunizationEvaluation resource
    pub resource_type: String,
    /// One possible path to achieve presumed immunity against a disease - within the
    /// context of an authority.
    pub series: super::string::String,
    /// The recommended number of doses to achieve immunity as determined by the outcome
    /// of the evaluation process.
    pub series_doses: super::string::String,
    /// Indicates the current status of the evaluation of the vaccination administration
    /// event.
    pub status: super::code::Code,
    /// The vaccine preventable disease the dose is being evaluated against.
    pub target_disease: super::codeable_concept::CodeableConcept,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}
