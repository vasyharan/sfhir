/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenario {
    /// A system or person who shares or receives an instance within the scenario.
    pub actor: Vec<super::example_scenario::ExampleScenarioActor>,
    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub contact: Vec<super::contact_detail::ContactDetail>,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// A copyright statement relating to the example scenario and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the example scenario.
    pub copyright: super::markdown::Markdown,
    /// A short string (<50 characters), suitable for inclusion in a page footer that
    /// identifies the copyright holder, effective period, and optionally whether rights
    /// are resctricted. (e.g. 'All rights reserved', 'Some rights reserved').
    pub copyright_label: super::string::String,
    /// The date  (and optionally time) when the example scenario was last significantly
    /// changed. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the example scenario changes. (e.g. the 'content logical
    /// definition').
    pub date: super::date_time::DateTime,
    /// A free text natural language description of the ExampleScenario from a
    /// consumer's perspective.
    pub description: super::markdown::Markdown,
    /// A Boolean value to indicate that this example scenario is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub experimental: super::boolean::Boolean,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// A formal identifier that is used to identify this example scenario when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A single data collection that is shared as part of the scenario.
    pub instance: Vec<super::example_scenario::ExampleScenarioInstance>,
    /// A legal or geographic region in which the example scenario is intended to be
    /// used.
    pub jurisdiction: Vec<super::codeable_concept::CodeableConcept>,
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
    /// Temporarily retained for tooling purposes.
    pub name: super::string::String,
    /// A group of operations that represents a significant step within a scenario.
    pub process: Vec<super::example_scenario::ExampleScenarioProcess>,
    /// The name of the organization or individual responsible for the release and
    /// ongoing maintenance of the example scenario.
    pub publisher: super::string::String,
    /// What the example scenario resource is created for. This should not be used to
    /// show the business purpose of the scenario itself, but the purpose of documenting
    /// a scenario.
    pub purpose: super::markdown::Markdown,
    /// This is a ExampleScenario resource
    pub resource_type: String,
    /// The status of this example scenario. Enables tracking the life-cycle of the
    /// content.
    pub status: super::code::Code,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
    /// A short, descriptive, user-friendly title for the ExampleScenario.
    pub title: super::string::String,
    /// An absolute URI that is used to identify this example scenario when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which an authoritative instance of this example scenario is (or will
    /// be) published. This URL can be the target of a canonical reference. It SHALL
    /// remain the same when the example scenario is stored on different servers.
    pub url: super::uri::Uri,
    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate example scenario
    /// instances.
    pub use_context: Vec<super::usage_context::UsageContext>,
    /// The identifier that is used to identify this version of the example scenario
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the example scenario author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub version: super::string::String,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_coding: super::coding::Coding,
    /// Indicates the mechanism used to compare versions to determine which is more
    /// current.
    pub version_algorithm_string: String,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioActor {
    /// An explanation of who/what the actor is and its role in the scenario.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A unique string within the scenario that is used to reference the actor.
    pub key: super::string::String,
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
    /// The human-readable name for the actor used when rendering the scenario.
    pub title: super::string::String,
    /// The category of actor - person or system.
    pub r#type: super::code::Code,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioAlternative {
    /// A human-readable description of the alternative explaining when the alternative
    /// should occur rather than the base step.
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
    /// Indicates the operation, sub-process or scenario that happens if the alternative
    /// option is selected.
    pub step: Vec<super::example_scenario::ExampleScenarioStep>,
    /// The label to display for the alternative that gives a sense of the circumstance
    /// in which the alternative should be invoked.
    pub title: super::string::String,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioContainedInstance {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A reference to the key of an instance found within this one.
    pub instance_reference: super::string::String,
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
    /// A reference to the key of a specific version of an instance in this instance.
    pub version_reference: super::string::String,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioInstance {
    /// References to other instances that can be found within this instance (e.g. the
    /// observations contained in a bundle).
    pub contained_instance: Vec<super::example_scenario::ExampleScenarioContainedInstance>,
    /// Points to an instance (typically an example) that shows the data that would
    /// corespond to this instance.
    pub content: super::reference::Reference,
    /// An explanation of what the instance contains and what it's for.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A unique string within the scenario that is used to reference the instance.
    pub key: super::string::String,
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
    /// Refers to a profile, template or other ruleset the instance adheres to.
    pub structure_profile_canonical: String,
    /// Refers to a profile, template or other ruleset the instance adheres to.
    pub structure_profile_uri: String,
    /// A code indicating the kind of data structure (FHIR resource or some other
    /// standard) this is an instance of.
    pub structure_type: super::coding::Coding,
    /// Conveys the version of the data structure instantiated.  I.e. what release of
    /// FHIR, X12, OpenEHR, etc. is instance compliant with.
    pub structure_version: super::string::String,
    /// A short descriptive label the instance to be used in tables or diagrams.
    pub title: super::string::String,
    /// Represents the instance as it was at a specific time-point.
    pub version: Vec<super::example_scenario::ExampleScenarioVersion>,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioOperation {
    /// An explanation of what the operation represents and what it does.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The system that invokes the action/transmits the data.
    pub initiator: super::string::String,
    /// If false, the initiator is deactivated right after the operation.
    pub initiator_active: super::boolean::Boolean,
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
    /// The system on which the action is invoked/receives the data.
    pub receiver: super::string::String,
    /// If false, the receiver is deactivated right after the operation.
    pub receiver_active: super::boolean::Boolean,
    /// A reference to the instance that is transmitted from requester to receiver as
    /// part of the invocation of the operation.
    pub request: super::example_scenario::ExampleScenarioContainedInstance,
    /// A reference to the instance that is transmitted from receiver to requester as
    /// part of the operation's synchronous response (if any).
    pub response: super::example_scenario::ExampleScenarioContainedInstance,
    /// A short descriptive label the step to be used in tables or diagrams.
    pub title: super::string::String,
    /// The standardized type of action (FHIR or otherwise).
    pub r#type: super::coding::Coding,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioProcess {
    /// An explanation of what the process represents and what it does.
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
    /// Description of the final state of the actors, environment and data after the
    /// process has been successfully completed.
    pub post_conditions: super::markdown::Markdown,
    /// Description of the initial state of the actors, environment and data before the
    /// process starts.
    pub pre_conditions: super::markdown::Markdown,
    /// A significant action that occurs as part of the process.
    pub step: Vec<super::example_scenario::ExampleScenarioStep>,
    /// A short descriptive label the process to be used in tables or diagrams.
    pub title: super::string::String,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioStep {
    /// Indicates an alternative step that can be taken instead of the sub-process,
    /// scenario or operation.  E.g. to represent non-happy-path/exceptional/atypical
    /// circumstances.
    pub alternative: Vec<super::example_scenario::ExampleScenarioAlternative>,
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
    /// The sequential number of the step, e.g. 1.2.5.
    pub number: super::string::String,
    /// The step represents a single operation invoked on receiver by sender.
    pub operation: super::example_scenario::ExampleScenarioOperation,
    /// If true, indicates that, following this step, there is a pause in the flow and
    /// the subsequent step will occur at some later time (triggered by some event).
    pub pause: super::boolean::Boolean,
    /// Indicates that the step is a complex sub-process with its own steps.
    pub process: super::example_scenario::ExampleScenarioProcess,
    /// Indicates that the step is defined by a seaparate scenario instance.
    pub workflow: super::canonical::Canonical,
}

/// Example of workflow instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleScenarioVersion {
    /// Points to an instance (typically an example) that shows the data that would flow
    /// at this point in the scenario.
    pub content: super::reference::Reference,
    /// An explanation of what this specific version of the instance contains and
    /// represents.
    pub description: super::markdown::Markdown,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A unique string within the instance that is used to reference the version of
    /// the instance.
    pub key: super::string::String,
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
    /// A short descriptive label the version to be used in tables or diagrams.
    pub title: super::string::String,
}
