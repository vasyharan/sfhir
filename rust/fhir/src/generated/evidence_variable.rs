/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariable {
    /// True if the actual variable measured, false if a conceptual representation of
    /// the intended variable.
    pub actual: super::boolean::Boolean,
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    ///
    /// See guidance around (not) making local changes to elements [here]
    /// (canonicalresource.html#localization).
    pub approval_date: super::date::Date,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// A grouping for ordinal or polychotomous variables.
    pub category: Vec<super::evidence_variable::EvidenceVariableCategory>,
    /// A defining factor of the EvidenceVariable. Multiple characteristics are applied
    /// with "and" semantics.
    pub characteristic: Vec<super::evidence_variable::EvidenceVariableCharacteristic>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the resource and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// resource.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the evidence variable was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the evidence variable changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the evidence variable from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the resource content was or is planned to be in active
    /// use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this resource is authored for testing purposes
    /// (or education/evaluation/marketing) and is not intended to be used for genuine
    /// usage.
    pub experimental: super::boolean::Boolean,
    /// The method of handling in statistical analysis.
    pub handling: super::code::Code,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this evidence variable when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the {{title}} is intended to be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
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
    /// A natural language name identifying the evidence variable. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// A human-readable string to clarify or explain concepts about the resource.
    pub note: Vec<super::annotation::Annotation>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the evidence variable.
    pub publisher: super::string::String,
    /// Explanation of why this EvidenceVariable is needed and why it has been designed
    /// as it has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a EvidenceVariable resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The short title provides an alternate title for use in informal descriptive
    /// contexts where the full, formal title is not necessary.
    pub short_title: super::string::String,
    /// The status of this evidence variable. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the evidence variable.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the {{title}}. Topics provide a
    /// high-level categorization as well as keywords for the {{title}} that can be
    /// useful for filtering and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// An absolute URI that is used to identify this evidence variable when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this evidence variable is (or will
    /// be) published. This URL can be the target of a canonical reference. It SHALL
    /// remain the same when the evidence variable is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate evidence variable
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the evidence variable
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the evidence variable author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence. To provide a version consistent with
    /// the Decision Support Service specification, use the format Major.Minor.Revision
    /// (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the
    /// Decision Support Service specification. Note that a version is required for non-
    /// experimental active artifacts.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCategory {
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
    /// Description of the grouping.
    pub name: super::string::String,
    /// Definition of the grouping.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Definition of the grouping.
    pub value_quantity: super::quantity::Quantity,
    /// Definition of the grouping.
    pub value_range: super::range::Range,
}

/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableCharacteristic {
    /// Defines the characteristic as a combination of two or more characteristics.
    pub definition_by_combination:
        super::evidence_variable::EvidenceVariableDefinitionByCombination,
    /// Defines the characteristic using both a type and value[x] elements.
    pub definition_by_type_and_value:
        super::evidence_variable::EvidenceVariableDefinitionByTypeAndValue,
    /// Defines the characteristic using Canonical.
    pub definition_canonical: super::canonical::Canonical,
    /// Defines the characteristic using CodeableConcept.
    pub definition_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Defines the characteristic using Expression.
    pub definition_expression: super::expression::Expression,
    /// Defines the characteristic using id.
    pub definition_id: super::id::Id,
    /// Defines the characteristic using a Reference.
    pub definition_reference: super::reference::Reference,
    /// A short, natural language description of the characteristic that could be used
    /// to communicate the criteria to an end-user.
    pub description: super::markdown::Markdown,
    /// Length of time in which the characteristic is met.
    pub duration_quantity: super::quantity::Quantity,
    /// Length of time in which the characteristic is met.
    pub duration_range: super::range::Range,
    /// When true, this characteristic is an exclusion criterion. In other words, not
    /// matching this characteristic definition is equivalent to meeting this criterion.
    pub exclude: super::boolean::Boolean,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Number of occurrences meeting the characteristic.
    pub instances_quantity: super::quantity::Quantity,
    /// Number of occurrences meeting the characteristic.
    pub instances_range: super::range::Range,
    /// Label used for when a characteristic refers to another characteristic.
    pub link_id: super::id::Id,
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
    /// A human-readable string to clarify or explain concepts about the characteristic.
    pub note: Vec<super::annotation::Annotation>,
    /// Timing in which the characteristic is determined.
    pub time_from_event: Vec<super::evidence_variable::EvidenceVariableTimeFromEvent>,
}

/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableDefinitionByCombination {
    /// A defining factor of the characteristic.
    pub characteristic: Vec<super::evidence_variable::EvidenceVariableCharacteristic>,
    /// Used to specify if two or more characteristics are combined with OR or AND.
    pub code: super::code::Code,
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
    /// Provides the value of "n" when "at-least" or "at-most" codes are used.
    pub threshold: super::positive_int::PositiveInt,
}

/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableDefinitionByTypeAndValue {
    /// Device used for determining characteristic.
    pub device: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Method for how the characteristic value was determined.
    pub method: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Defines the reference point for comparison when valueQuantity or valueRange is
    /// not compared to zero.
    pub offset: super::codeable_concept::CodeableConcept,
    /// Used to express the type of characteristic.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// Defines the characteristic when paired with characteristic.type.
    pub value_boolean: bool,
    /// Defines the characteristic when paired with characteristic.type.
    pub value_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Defines the characteristic when paired with characteristic.type.
    pub value_id: String,
    /// Defines the characteristic when paired with characteristic.type.
    pub value_quantity: super::quantity::Quantity,
    /// Defines the characteristic when paired with characteristic.type.
    pub value_range: super::range::Range,
    /// Defines the characteristic when paired with characteristic.type.
    pub value_reference: super::reference::Reference,
}

/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceVariableTimeFromEvent {
    /// Human readable description.
    pub description: super::markdown::Markdown,
    /// The event used as a base point (reference point) in time.
    pub event_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The event used as a base point (reference point) in time.
    pub event_date_time: String,
    /// The event used as a base point (reference point) in time.
    pub event_id: String,
    /// The event used as a base point (reference point) in time.
    pub event_reference: super::reference::Reference,
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
    /// A human-readable string to clarify or explain concepts about the timeFromEvent.
    pub note: Vec<super::annotation::Annotation>,
    /// Used to express the observation at a defined amount of time before or after
    /// the event.
    pub quantity: super::quantity::Quantity,
    /// Used to express the observation within a period before and/or after the event.
    pub range: super::range::Range,
}
