/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestration {
    /// The actions, if any, produced by the evaluation of the artifact.
    pub action: Vec<super::request_orchestration::RequestOrchestrationAction>,
    /// Provides a reference to the author of the request orchestration.
    pub author: super::reference::Reference,
    /// Indicates when the request orchestration was created.
    pub authored_on: super::date_time::DateTime,
    /// A plan, proposal or order that is fulfilled in whole or in part by this request.
    pub based_on: Vec<super::reference::Reference>,
    /// A code that identifies what the overall request orchestration is.
    pub code: super::codeable_concept::CodeableConcept,
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, nor can they have
    /// their own independent transaction scope. This is allowed to be a Parameters
    /// resource if and only if it is referenced by a resource that provides context/
    /// meaning.
    pub contained: Vec<super::resource_list::ResourceList>,
    /// Describes the context of the request orchestration, if any.
    pub encounter: super::reference::Reference,
    /// Goals that are intended to be achieved by following the requests in this
    /// RequestOrchestration.
    pub goal: Vec<super::reference::Reference>,
    /// A shared identifier common to multiple independent Request instances that
    /// were activated/authorized more or less simultaneously by a single author.  The
    /// presence of the same identifier on each request ties those requests together and
    /// may have business ramifications in terms of reporting of results, billing, etc.
    /// E.g. a requisition number shared by a set of lab tests ordered together, or a
    /// prescription number shared by all meds ordered at one time.
    pub group_identifier: super::identifier::Identifier,
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub id: super::id::Id,
    /// Allows a service to provide a unique, business identifier for the request.
    pub identifier: Vec<super::identifier::Identifier>,
    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub implicit_rules: super::uri::Uri,
    /// A canonical URL referencing a FHIR-defined protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this request.
    pub instantiates_canonical: Vec<super::canonical::Canonical>,
    /// A URL referencing an externally defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this request.
    pub instantiates_uri: Vec<super::uri::Uri>,
    /// Indicates the level of authority/intentionality associated with the request and
    /// where the request fits into the workflow chain.
    pub intent: super::code::Code,
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
    /// Provides a mechanism to communicate additional information about the response.
    pub note: Vec<super::annotation::Annotation>,
    /// Indicates how quickly the request should be addressed with respect to other
    /// requests.
    pub priority: super::code::Code,
    /// Describes the reason for the request orchestration in coded or textual form.
    pub reason: Vec<super::codeable_reference::CodeableReference>,
    /// Completed or terminated request(s) whose function is taken by this new request.
    pub replaces: Vec<super::reference::Reference>,
    /// This is a RequestOrchestration resource
    pub resource_type: String,
    /// The current state of the request. For request orchestrations, the status
    /// reflects the status of all the requests in the orchestration.
    pub status: super::code::Code,
    /// The subject for which the request orchestration was created.
    pub subject: super::reference::Reference,
    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need
    /// not encode all the structured data, but is required to contain sufficient detail
    /// to make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub text: super::narrative::Narrative,
}

/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationAction {
    /// Sub actions.
    pub action: Vec<super::request_orchestration::RequestOrchestrationAction>,
    /// Defines whether the action can be selected multiple times.
    pub cardinality_behavior: super::code::Code,
    /// A code that provides meaning for the action or action group. For example, a
    /// section may have a LOINC code for a section of a documentation template.
    pub code: Vec<super::codeable_concept::CodeableConcept>,
    /// An expression that describes applicability criteria, or start/stop conditions
    /// for the action.
    pub condition: Vec<super::request_orchestration::RequestOrchestrationCondition>,
    /// A reference to an ActivityDefinition that describes the action to be taken
    /// in detail, a PlanDefinition that describes a series of actions to be taken,
    /// a Questionnaire that should be filled out, a SpecimenDefinition describing
    /// a specimen to be collected, or an ObservationDefinition that specifies what
    /// observation should be captured.
    pub definition_canonical: String,
    /// A reference to an ActivityDefinition that describes the action to be taken
    /// in detail, a PlanDefinition that describes a series of actions to be taken,
    /// a Questionnaire that should be filled out, a SpecimenDefinition describing
    /// a specimen to be collected, or an ObservationDefinition that specifies what
    /// observation should be captured.
    pub definition_uri: String,
    /// A short description of the action used to provide a summary to display to the
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
    pub dynamic_value: Vec<super::request_orchestration::RequestOrchestrationDynamicValue>,
    /// Goals that are intended to be achieved by following the requests in this action.
    pub goal: Vec<super::reference::Reference>,
    /// Defines the grouping behavior for the action and its children.
    pub grouping_behavior: super::code::Code,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Defines input data requirements for the action.
    pub input: Vec<super::request_orchestration::RequestOrchestrationInput>,
    /// The linkId of the action from the PlanDefinition that corresponds to this action
    /// in the RequestOrchestration resource.
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
    pub output: Vec<super::request_orchestration::RequestOrchestrationOutput>,
    /// The participant that should perform or be responsible for this action.
    pub participant: Vec<super::request_orchestration::RequestOrchestrationParticipant>,
    /// Defines whether the action should usually be preselected.
    pub precheck_behavior: super::code::Code,
    /// A user-visible prefix for the action. For example a section or item numbering
    /// such as 1. or A.
    pub prefix: super::string::String,
    /// Indicates how quickly the action should be addressed with respect to other
    /// actions.
    pub priority: super::code::Code,
    /// A relationship to another action such as "before" or "30-60 minutes after start
    /// of".
    pub related_action: Vec<super::request_orchestration::RequestOrchestrationRelatedAction>,
    /// Defines expectations around whether an action is required.
    pub required_behavior: super::code::Code,
    /// The resource that is the target of the action (e.g. CommunicationRequest).
    pub resource: super::reference::Reference,
    /// Defines the selection behavior for the action and its children.
    pub selection_behavior: super::code::Code,
    /// A text equivalent of the action to be performed. This provides a human-
    /// interpretable description of the action when the definition is consumed by a
    /// system that might not be capable of interpreting it dynamically.
    pub text_equivalent: super::markdown::Markdown,
    /// An optional value describing when the action should be performed.
    pub timing_age: super::age::Age,
    /// An optional value describing when the action should be performed.
    pub timing_date_time: String,
    /// An optional value describing when the action should be performed.
    pub timing_duration: super::duration::Duration,
    /// An optional value describing when the action should be performed.
    pub timing_period: super::period::Period,
    /// An optional value describing when the action should be performed.
    pub timing_range: super::range::Range,
    /// An optional value describing when the action should be performed.
    pub timing_timing: super::timing::Timing,
    /// The title of the action displayed to a user.
    pub title: super::string::String,
    /// A reference to a StructureMap resource that defines a transform that can be
    /// executed to produce the intent resource using the ActivityDefinition instance as
    /// the input.
    pub transform: super::canonical::Canonical,
    /// The type of action to perform (create, update, remove).
    pub r#type: super::codeable_concept::CodeableConcept,
}

/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationCondition {
    /// An expression that returns true or false, indicating whether or not the
    /// condition is satisfied.
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

/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationDynamicValue {
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

/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationInput {
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

/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationOutput {
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

/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationParticipant {
    /// A reference to the actual participant.
    pub actor_canonical: String,
    /// A reference to the actual participant.
    pub actor_reference: super::reference::Reference,
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

/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, PartialEq)]
pub struct RequestOrchestrationRelatedAction {
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
    /// The relationship of this action to the related action.
    pub relationship: super::code::Code,
    /// The element id of the target related action.
    pub target_id: super::id::Id,
}
