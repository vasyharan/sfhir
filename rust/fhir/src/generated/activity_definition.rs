/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance context.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivityDefinition {
    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub approval_date: super::date::Date,
    /// If a CodeableConcept is present, it indicates the pre-condition for performing
    /// the service.  For example "pain", "on flare-up", etc.
    pub as_needed_boolean: bool,
    /// If a CodeableConcept is present, it indicates the pre-condition for performing
    /// the service.  For example "pain", "on flare-up", etc.
    pub as_needed_codeable_concept: super::codeable_concept::CodeableConcept,
    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub author: Vec<super::contact_detail::ContactDetail>,
    /// Indicates the sites on the subject's body where the procedure should be
    /// performed (I.e. the target sites).
    pub body_site: Vec<super::codeable_concept::CodeableConcept>,
    /// Detailed description of the type of activity; e.g. What lab test, what
    /// procedure, what kind of encounter.
    pub code: super::codeable_concept::CodeableConcept,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the activity definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the activity definition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the activity definition was last
    /// significantly changed. The date must change when the business version changes
    /// and it must change if the status code changes. In addition, it should change
    /// when the substantive content of the activity definition changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the activity definition from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// Set this to true if the definition is to indicate that a particular activity
    /// should NOT be performed. If true, this element should be interpreted to
    /// reinforce a negative coding. For example NPO as a code with a doNotPerform of
    /// true would still indicate to NOT perform the action.
    pub do_not_perform: super::boolean::Boolean,
    /// Provides detailed dosage instructions in the same way that they are described
    /// for MedicationRequest resources.
    pub dosage: Vec<super::dosage::Dosage>,
    /// Dynamic values that will be evaluated to produce values for elements of the
    /// resulting resource. For example, if the dosage of a medication must be computed
    /// based on the patient's weight, a dynamic value would be used to specify an
    /// expression that calculated the weight, and the path on the request resource that
    /// would contain the result.
    pub dynamic_value: Vec<super::activity_definition::ActivityDefinitionDynamicValue>,
    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the activity definition content was or is planned to be
    /// in active use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this activity definition is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended to be
    /// used for genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this activity definition when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// Indicates the level of authority/intentionality associated with the activity and
    /// where the request should fit into the workflow chain.
    pub intent: super::code::Code,
    /// A legal or geographic region in which the activity definition is intended to
    /// be used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// A description of the kind of resource the activity definition is representing.
    /// For example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest.
    pub kind: super::code::Code,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub last_review_date: super::date::Date,
    /// A reference to a Library resource containing any formal logic used by the
    /// activity definition.
    pub library: Vec<super::canonical::Canonical>,
    /// Identifies the facility where the activity will occur; e.g. home, hospital,
    /// specific clinic, etc.
    pub location: super::codeable_reference::CodeableReference,
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
    /// A natural language name identifying the activity definition. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// Defines observation requirements for the action to be performed, such as body
    /// weight or surface area.
    pub observation_requirement: Vec<super::canonical::Canonical>,
    /// Defines the observations that are expected to be produced by the action.
    pub observation_result_requirement: Vec<super::canonical::Canonical>,
    /// Indicates who should participate in performing the action described.
    pub participant: Vec<super::activity_definition::ActivityDefinitionParticipant>,
    /// Indicates how quickly the activity  should be addressed with respect to other
    /// requests.
    pub priority: super::code::Code,
    /// Identifies the food, drug or other product being consumed or supplied in the
    /// activity.
    pub product_codeable_concept: super::codeable_concept::CodeableConcept,
    /// Identifies the food, drug or other product being consumed or supplied in the
    /// activity.
    pub product_reference: super::reference::Reference,
    /// A profile to which the target of the activity definition is expected to conform.
    pub profile: super::canonical::Canonical,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the activity definition.
    pub publisher: super::string::String,
    /// Explanation of why this activity definition is needed and why it has been
    /// designed as it has.
    pub purpose: super::markdown::Markdown,
    /// Identifies the quantity expected to be consumed at once (per dose, per meal,
    /// etc.).
    pub quantity: super::quantity::Quantity,
    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a ActivityDefinition resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// Defines specimen requirements for the action to be performed, such as required
    /// specimens for a lab test.
    pub specimen_requirement: Vec<super::canonical::Canonical>,
    /// The status of this activity definition. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A code, group definition, or canonical reference that describes  or identifies
    /// the intended subject of the activity being defined.  Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_canonical: String,
    /// A code, group definition, or canonical reference that describes  or identifies
    /// the intended subject of the activity being defined.  Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A code, group definition, or canonical reference that describes  or identifies
    /// the intended subject of the activity being defined.  Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_reference: super::reference::Reference,
    /// An explanatory or alternate title for the activity definition giving additional
    /// information about its content.
    pub subtitle: super::string::String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// The timing or frequency upon which the described activity is to occur.
    pub timing_age: super::age::Age,
    /// The timing or frequency upon which the described activity is to occur.
    pub timing_duration: super::duration::Duration,
    /// The timing or frequency upon which the described activity is to occur.
    pub timing_range: super::range::Range,
    /// The timing or frequency upon which the described activity is to occur.
    pub timing_timing: super::timing::Timing,
    /// A short, descriptive, user-friendly title for the activity definition.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the activity. Topics provide a
    /// high-level categorization of the activity that can be useful for filtering and
    /// searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// A reference to a StructureMap resource that defines a transform that can be
    /// executed to produce the intent resource using the ActivityDefinition instance as
    /// the input.
    pub transform: super::canonical::Canonical,
    /// An absolute URI that is used to identify this activity definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this activity definition is
    /// (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the activity definition is stored on different
    /// servers.
    pub url: super::uri::Uri,
    /// A detailed description of how the activity definition is used from a clinical
    /// perspective.
    pub usage: super::markdown::Markdown,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...)
    /// or may be references to specific programs (insurance plans, studies, ...) and
    /// may be used to assist with indexing and searching for appropriate activity
    /// definition instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the activity definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the activity definition author and is not expected
    /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence. To provide a version consistent with
    /// the Decision Support Service specification, use the format Major.Minor.Revision
    /// (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the
    /// Decision Support Service specification. Note that a version is required for non-
    /// experimental active assets.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance context.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivityDefinitionDynamicValue {
    /// An expression specifying the value of the customized element.
    pub expression: super::expression::Expression,
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
    /// The path to the element to be customized. This is the path on the resource
    /// that will hold the result of the calculation defined by the expression. The
    /// specified path SHALL be a FHIRPath resolvable on the specified target type
    /// of the ActivityDefinition, and SHALL consist only of identifiers, constant
    /// indexers, and a restricted subset of functions. The path is allowed to
    /// contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to
    /// traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile]
    /// (fhirpath.html#simple) for full details).
    pub path: super::string::String,
}

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance context.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivityDefinitionParticipant {
    /// Indicates how the actor will be involved in the action - author, reviewer,
    /// witness, etc.
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
    /// The role the participant should play in performing the described action.
    pub role: super::codeable_concept::CodeableConcept,
    /// The type of participant in the action.
    pub r#type: super::code::Code,
    /// The type of participant in the action.
    pub type_canonical: super::canonical::Canonical,
    /// The type of participant in the action.
    pub type_reference: super::reference::Reference,
}
