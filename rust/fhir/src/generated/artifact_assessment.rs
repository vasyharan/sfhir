/// This Resource provides one or more comments, classifiers or ratings about a
/// Resource and supports attribution and rights management metadata for the added
/// content.
#[derive(Debug, Clone, PartialEq)]
pub struct ArtifactAssessment {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// A reference to a resource, canonical resource, or non-FHIR resource which the
    /// comment or assessment is about.
    pub artifact_canonical: String,
    /// A reference to a resource, canonical resource, or non-FHIR resource which the
    /// comment or assessment is about.
    pub artifact_reference: super::reference::Reference,
    /// A reference to a resource, canonical resource, or non-FHIR resource which the
    /// comment or assessment is about.
    pub artifact_uri: String,
    /// Display of or reference to the bibliographic citation of the comment,
    /// classifier, or rating.
    pub cite_as_markdown: String,
    /// Display of or reference to the bibliographic citation of the comment,
    /// classifier, or rating.
    pub cite_as_reference: super::reference::Reference,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A component comment, classifier, or rating of the artifact.
    pub content: Vec<super::artifact_assessment::ArtifactAssessmentContent>,
    /// A copyright statement relating to the artifact assessment and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the artifact assessment.
    pub copyright: super::markdown::Markdown,
    /// The date  (and optionally time) when the artifact assessment was published. The
    /// date must change when the disposition changes and it must change if the workflow
    /// status code changes. In addition, it should change when the substantive content
    /// of the artifact assessment changes.
    pub date: super::date_time::DateTime,
    /// Indicates the disposition of the responsible party to the comment or change
    /// request.
    pub disposition: super::code::Code,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this artifact assessment when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub last_review_date: super::date::Date,
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
    /// This is a ArtifactAssessment resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short title for the assessment for use in displaying and selecting.
    pub title: super::string::String,
    /// Indicates the workflow status of the comment or change request.
    pub workflow_status: super::code::Code,
}

/// This Resource provides one or more comments, classifiers or ratings about a
/// Resource and supports attribution and rights management metadata for the added
/// content.
#[derive(Debug, Clone, PartialEq)]
pub struct ArtifactAssessmentContent {
    /// Indicates who or what authored the content.
    pub author: super::reference::Reference,
    /// Represents a rating, classifier, or assessment of the artifact.
    pub classifier: Vec<super::codeable_concept::CodeableConcept>,
    /// If the informationType is container, the components of the content.
    pub component: Vec<super::artifact_assessment::ArtifactAssessmentContent>,
    /// Acceptable to publicly share the comment, classifier or rating.
    pub free_to_share: super::boolean::Boolean,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The type of information this component of the content represents.
    pub information_type: super::code::Code,
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
    /// A URI that points to what the comment is about, such as a line of text in the
    /// CQL, or a specific element in a resource.
    pub path: Vec<super::uri::Uri>,
    /// A quantitative rating of the artifact.
    pub quantity: super::quantity::Quantity,
    /// Additional related artifacts that provide supporting documentation, additional
    /// evidence, or further information related to the content.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// A brief summary of the content of this component.
    pub summary: super::markdown::Markdown,
    /// Indicates what type of content this component represents.
    pub r#type: super::codeable_concept::CodeableConcept,
}
