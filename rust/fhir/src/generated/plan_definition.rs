/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinition {
    /// An action or group of actions to be taken as part of the plan. For example,
    /// in clinical care, an action would be to prescribe a particular indicated
    /// medication, or perform a particular test as appropriate. In pharmaceutical
    /// quality, an action would be the test that needs to be performed on a drug
    /// product as defined in the quality specification.
    pub action: Vec<super::plan_definition::PlanDefinitionAction>,
    /// Actors represent the individuals or groups involved in the execution of the
    /// defined set of activities.
    pub actor: Vec<super::plan_definition::PlanDefinitionActor>,
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
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the plan definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the plan definition.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the plan definition was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the plan definition changes.
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the plan definition from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub editor: Vec<super::contact_detail::ContactDetail>,
    /// The period during which the plan definition content was or is planned to be in
    /// active use.
    pub effective_period: super::period::Period,
    /// An individual or organization asserted by the publisher to be responsible for
    /// officially endorsing the content for use in some setting.
    pub endorser: Vec<super::contact_detail::ContactDetail>,
    /// A Boolean value to indicate that this plan definition is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// A goal describes an expected outcome that activities within the plan are
    /// intended to achieve. For example, weight loss, restoring an activity of daily
    /// living, obtaining herd immunity via immunization, meeting a process improvement
    /// objective, meeting the acceptance criteria for a test as specified by a quality
    /// specification, etc.
    pub goal: Vec<super::plan_definition::PlanDefinitionGoal>,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this plan definition when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A legal or geographic region in which the plan definition is intended to be
    /// used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
    /// The base language in which the resource is written.
    pub language: super::code::Code,
    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub last_review_date: super::date::Date,
    /// A reference to a Library resource containing any formal logic used by the plan
    /// definition.
    pub library: Vec<super::canonical::Canonical>,
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
    /// A natural language name identifying the plan definition. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub name: super::string::String,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the plan definition.
    pub publisher: super::string::String,
    /// Explanation of why this plan definition is needed and why it has been designed
    /// as it has.
    pub purpose: super::markdown::Markdown,
    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub related_artifact: Vec<super::related_artifact::RelatedArtifact>,
    /// This is a PlanDefinition resource
    pub resource_type: String,
    /// An individual or organization asserted by the publisher to be primarily
    /// responsible for review of some aspect of the content.
    pub reviewer: Vec<super::contact_detail::ContactDetail>,
    /// The status of this plan definition. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A code, group definition, or canonical reference that describes  or identifies
    /// the intended subject of the plan definition. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_canonical: String,
    /// A code, group definition, or canonical reference that describes  or identifies
    /// the intended subject of the plan definition. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A code, group definition, or canonical reference that describes  or identifies
    /// the intended subject of the plan definition. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_reference: super::reference::Reference,
    /// An explanatory or alternate title for the plan definition giving additional
    /// information about its content.
    pub subtitle: super::string::String,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the plan definition.
    pub title: super::string::String,
    /// Descriptive topics related to the content of the plan definition. Topics provide
    /// a high-level categorization of the definition that can be useful for filtering
    /// and searching.
    pub topic: Vec<super::codeable_concept::CodeableConcept>,
    /// A high-level category for the plan definition that distinguishes the kinds of
    /// systems that would be interested in the plan definition.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// An absolute URI that is used to identify this plan definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this plan definition is (or will
    /// be) published. This URL can be the target of a canonical reference. It SHALL
    /// remain the same when the plan definition is stored on different servers.
    pub url: super::uri::Uri,
    /// A detailed description of how the plan definition is used from a clinical
    /// perspective.
    pub usage: super::markdown::Markdown,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate plan definition
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the plan definition
    /// when it is referenced in a specification, model, design or instance. This is
    /// an arbitrary value managed by the plan definition author and is not expected to
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

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionAction {
    /// Sub actions that are contained within the action. The behavior of this action
    /// determines the functionality of the sub-actions. For example, a selection
    /// behavior of at-most-one indicates that of the sub-actions, at most one may be
    /// chosen as part of realizing the action definition.
    pub action: Vec<super::plan_definition::PlanDefinitionAction>,
    /// Defines whether the action can be selected multiple times.
    pub cardinality_behavior: super::code::Code,
    /// A code that provides a meaning, grouping, or classification for the action or
    /// action group. For example, a section may have a LOINC code for the section of a
    /// documentation template. In pharmaceutical quality, an action (Test) such as pH
    /// could be classified as a physical property.
    pub code: super::codeable_concept::CodeableConcept,
    /// An expression that describes applicability criteria or start/stop conditions for
    /// the action.
    pub condition: Vec<super::plan_definition::PlanDefinitionCondition>,
    /// A reference to an ActivityDefinition that describes the action to be taken in
    /// detail, a MessageDefinition describing a message to be snet, a PlanDefinition
    /// that describes a series of actions to be taken, a Questionnaire that should be
    /// filled out, a SpecimenDefinition describing a specimen to be collected, or an
    /// ObservationDefinition that specifies what observation should be captured.
    pub definition_canonical: String,
    /// A reference to an ActivityDefinition that describes the action to be taken in
    /// detail, a MessageDefinition describing a message to be snet, a PlanDefinition
    /// that describes a series of actions to be taken, a Questionnaire that should be
    /// filled out, a SpecimenDefinition describing a specimen to be collected, or an
    /// ObservationDefinition that specifies what observation should be captured.
    pub definition_uri: String,
    /// A brief description of the action used to provide a summary to display to the
    /// user.
    pub description: super::markdown::Markdown,
    /// Didactic or other informational resources associated with the action that can
    /// be provided to the CDS recipient. Information resources can include inline text
    /// commentary and links to web resources.
    pub documentation: Vec<super::related_artifact::RelatedArtifact>,
    /// Customizations that should be applied to the statically defined resource. For
    /// example, if the dosage of a medication must be computed based on the patient's
    /// weight, a customization would be used to specify an expression that calculated
    /// the weight, and the path on the resource that would contain the result.
    pub dynamic_value: Vec<super::plan_definition::PlanDefinitionDynamicValue>,
    /// Identifies goals that this action supports. The reference must be to a goal
    /// element defined within this plan definition. In pharmaceutical quality, a goal
    /// represents acceptance criteria (Goal) for a given action (Test), so the goalId
    /// would be the unique id of a defined goal element establishing the acceptance
    /// criteria for the action.
    pub goal_id: Vec<super::id::Id>,
    /// Defines the grouping behavior for the action and its children.
    pub grouping_behavior: super::code::Code,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Defines input data requirements for the action.
    pub input: Vec<super::plan_definition::PlanDefinitionInput>,
    /// An identifier that is unique within the PlanDefinition to allow linkage within
    /// the realized CarePlan and/or RequestOrchestration.
    pub link_id: super::string::String,
    /// Identifies the facility where the action will occur; e.g. home, hospital,
    /// specific clinic, etc.
    pub location: super::codeable_reference::CodeableReference,
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
    /// Defines the outputs of the action, if any.
    pub output: Vec<super::plan_definition::PlanDefinitionOutput>,
    /// Indicates who should participate in performing the action described.
    pub participant: Vec<super::plan_definition::PlanDefinitionParticipant>,
    /// Defines whether the action should usually be preselected.
    pub precheck_behavior: super::code::Code,
    /// A user-visible prefix for the action. For example a section or item numbering
    /// such as 1. or A.
    pub prefix: super::string::String,
    /// Indicates how quickly the action should be addressed with respect to other
    /// actions.
    pub priority: super::code::Code,
    /// A description of why this action is necessary or appropriate.
    pub reason: Vec<super::codeable_concept::CodeableConcept>,
    /// A relationship to another action such as "before" or "30-60 minutes after start
    /// of".
    pub related_action: Vec<super::plan_definition::PlanDefinitionRelatedAction>,
    /// Defines the required behavior for the action.
    pub required_behavior: super::code::Code,
    /// Defines the selection behavior for the action and its children.
    pub selection_behavior: super::code::Code,
    /// A code, group definition, or canonical reference that describes the intended
    /// subject of the action and its children, if any. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_canonical: String,
    /// A code, group definition, or canonical reference that describes the intended
    /// subject of the action and its children, if any. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_codeable_concept: super::codeable_concept::CodeableConcept,
    /// A code, group definition, or canonical reference that describes the intended
    /// subject of the action and its children, if any. Canonical references are
    /// allowed to support the definition of protocols for drug and substance quality
    /// specifications, and is allowed to reference a MedicinalProductDefinition,
    /// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
    /// or PackagedProductDefinition resource.
    pub subject_reference: super::reference::Reference,
    /// A text equivalent of the action to be performed. This provides a human-
    /// interpretable description of the action when the definition is consumed by a
    /// system that might not be capable of interpreting it dynamically.
    pub text_equivalent: super::markdown::Markdown,
    /// An optional value describing when the action should be performed.
    pub timing_age: super::age::Age,
    /// An optional value describing when the action should be performed.
    pub timing_duration: super::duration::Duration,
    /// An optional value describing when the action should be performed.
    pub timing_range: super::range::Range,
    /// An optional value describing when the action should be performed.
    pub timing_timing: super::timing::Timing,
    /// The textual description of the action displayed to a user. For example, when the
    /// action is a test to be performed, the title would be the title of the test such
    /// as Assay by HPLC.
    pub title: super::string::String,
    /// A reference to a StructureMap resource that defines a transform that can be
    /// executed to produce the intent resource using the ActivityDefinition instance as
    /// the input.
    pub transform: super::canonical::Canonical,
    /// A description of when the action should be triggered. When multiple triggers are
    /// specified on an action, any triggering event invokes the action.
    pub trigger: Vec<super::trigger_definition::TriggerDefinition>,
    /// The type of action to perform (create, update, remove).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionActor {
    /// A description of how the actor fits into the overall actions of the plan
    /// definition.
    pub description: super::markdown::Markdown,
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
    /// The characteristics of the candidates that could serve as the actor.
    pub option: Vec<super::plan_definition::PlanDefinitionOption>,
    /// A descriptive label for the actor.
    pub title: super::string::String,
}

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionCondition {
    /// An expression that returns true or false, indicating whether the condition is
    /// satisfied.
    pub expression: super::expression::Expression,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The kind of condition.
    pub kind: super::code::Code,
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

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionDynamicValue {
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

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionGoal {
    /// Identifies problems, conditions, issues, or concerns the goal is intended to
    /// address.
    pub addresses: Vec<super::codeable_concept::CodeableConcept>,
    /// Indicates a category the goal falls within.
    pub category: super::codeable_concept::CodeableConcept,
    /// Human-readable and/or coded description of a specific desired objective of care,
    /// such as "control blood pressure" or "negotiate an obstacle course" or "dance
    /// with child at wedding".
    pub description: super::codeable_concept::CodeableConcept,
    /// Didactic or other informational resources associated with the goal that provide
    /// further supporting information about the goal. Information resources can include
    /// inline text commentary and links to web resources.
    pub documentation: Vec<super::related_artifact::RelatedArtifact>,
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
    /// Identifies the expected level of importance associated with reaching/sustaining
    /// the defined goal.
    pub priority: super::codeable_concept::CodeableConcept,
    /// The event after which the goal should begin being pursued.
    pub start: super::codeable_concept::CodeableConcept,
    /// Indicates what should be done and within what timeframe.
    pub target: Vec<super::plan_definition::PlanDefinitionTarget>,
}

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionInput {
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
    /// Points to an existing input or output element that provides data to this input.
    pub related_data: super::id::Id,
    /// Defines the data that is to be provided as input to the action.
    pub requirement: super::data_requirement::DataRequirement,
    /// A human-readable label for the data requirement used to label data flows in BPMN
    /// or similar diagrams. Also provides a human readable label when rendering the
    /// data requirement that conveys its purpose to human readers.
    pub title: super::string::String,
}

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionOption {
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

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionOutput {
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
    /// Points to an existing input or output element that is results as output from
    /// the action.
    pub related_data: super::string::String,
    /// Defines the data that results as output from the action.
    pub requirement: super::data_requirement::DataRequirement,
    /// A human-readable label for the data requirement used to label data flows in BPMN
    /// or similar diagrams. Also provides a human readable label when rendering the
    /// data requirement that conveys its purpose to human readers.
    pub title: super::string::String,
}

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionParticipant {
    /// A reference to the id element of the actor who will participate in this action.
    pub actor_id: super::string::String,
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

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionRelatedAction {
    /// The relationship of the end of this action to the related action.
    pub end_relationship: super::code::Code,
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
    /// A duration or range of durations to apply to the relationship. For example, 30-
    /// 60 minutes before.
    pub offset_duration: super::duration::Duration,
    /// A duration or range of durations to apply to the relationship. For example, 30-
    /// 60 minutes before.
    pub offset_range: super::range::Range,
    /// The relationship of the start of this action to the related action.
    pub relationship: super::code::Code,
    /// The element id of the target related action.
    pub target_id: super::id::Id,
}

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug, Clone, PartialEq)]
pub struct PlanDefinitionTarget {
    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT
    /// 0.6%, Clear solution, etc. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any value at or below the high value. Similarly, if the high value
    /// is missing, it indicates that the goal is achieved at any value at or above the
    /// low value.
    pub detail_boolean: bool,
    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT
    /// 0.6%, Clear solution, etc. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any value at or below the high value. Similarly, if the high value
    /// is missing, it indicates that the goal is achieved at any value at or above the
    /// low value.
    pub detail_codeable_concept: super::codeable_concept::CodeableConcept,
    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT
    /// 0.6%, Clear solution, etc. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any value at or below the high value. Similarly, if the high value
    /// is missing, it indicates that the goal is achieved at any value at or above the
    /// low value.
    pub detail_integer: i64,
    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT
    /// 0.6%, Clear solution, etc. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any value at or below the high value. Similarly, if the high value
    /// is missing, it indicates that the goal is achieved at any value at or above the
    /// low value.
    pub detail_quantity: super::quantity::Quantity,
    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT
    /// 0.6%, Clear solution, etc. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any value at or below the high value. Similarly, if the high value
    /// is missing, it indicates that the goal is achieved at any value at or above the
    /// low value.
    pub detail_range: super::range::Range,
    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT
    /// 0.6%, Clear solution, etc. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any value at or below the high value. Similarly, if the high value
    /// is missing, it indicates that the goal is achieved at any value at or above the
    /// low value.
    pub detail_ratio: super::ratio::Ratio,
    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT
    /// 0.6%, Clear solution, etc. Either the high or low or both values of the range
    /// can be specified. When a low value is missing, it indicates that the goal is
    /// achieved at any value at or below the high value. Similarly, if the high value
    /// is missing, it indicates that the goal is achieved at any value at or above the
    /// low value.
    pub detail_string: String,
    /// Indicates the timeframe after the start of the goal in which the goal should
    /// be met.
    pub due: super::duration::Duration,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The parameter whose value is to be tracked, e.g. body weight, blood pressure, or
    /// hemoglobin A1c level.
    pub measure: super::codeable_concept::CodeableConcept,
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
