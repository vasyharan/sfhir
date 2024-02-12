/// A collection of error, warning, or information messages that result from a
/// system action.
#[derive(Debug, Clone, PartialEq)]
pub struct OperationOutcome {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// An error, warning, or information message that results from a system action.
    pub issue: Vec<super::operation_outcome::OperationOutcomeIssue>,
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
    /// This is a OperationOutcome resource
    pub resource_type: String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A collection of error, warning, or information messages that result from a
/// system action.
#[derive(Debug, Clone, PartialEq)]
pub struct OperationOutcomeIssue {
    /// Describes the type of the issue. The system that creates an OperationOutcome
    /// SHALL choose the most applicable code from the IssueType value set, and may
    /// additional provide its own code for the error in the details element.
    pub code: super::code::Code,
    /// Additional details about the error. This may be a text description of the error
    /// or a system code that identifies the error.
    pub details: super::codeable_concept::CodeableConcept,
    /// Additional diagnostic information about the issue.
    pub diagnostics: super::string::String,
    /// A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names,
    /// repetition indicators and the default child accessor that identifies one of the
    /// elements in the resource that caused this issue to be raised.
    pub expression: Vec<super::string::String>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// This element is deprecated because it is XML specific. It is replaced by
    /// issue.expression, which is format independent, and simpler to parse.
    ///
    /// For resource issues, this will be a simple XPath limited to element names,
    /// repetition indicators and the default child accessor that identifies one of the
    /// elements in the resource that caused this issue to be raised.  For HTTP errors,
    /// will be "http." + the parameter name.
    pub location: Vec<super::string::String>,
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
    /// Indicates whether the issue indicates a variation from successful processing.
    pub severity: super::code::Code,
}
