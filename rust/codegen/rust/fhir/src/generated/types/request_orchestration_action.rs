use super::*;
/// A set of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug,Clone,PartialEq)]
pub struct RequestOrchestrationAction {
/// Sub actions.
pub action: Vec<RequestOrchestrationAction>,
/// Defines whether the action can be selected multiple times.
pub cardinality_behavior: Code,
/// A code that provides meaning for the action or action group. For example, a
/// section may have a LOINC code for a section of a documentation template.
pub code: Vec<CodeableConcept>,
/// An expression that describes applicability criteria, or start/stop conditions
/// for the action.
pub condition: Vec<RequestOrchestrationCondition>,
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
pub description: Markdown,
/// Didactic or other informational resources associated with the action that can
/// be provided to the CDS recipient. Information resources can include inline text
/// commentary and links to web resources.
pub documentation: Vec<RelatedArtifact>,
/// Customizations that should be applied to the statically defined resource. For
/// example, if the dosage of a medication must be computed based on the patient's
/// weight, a customization would be used to specify an expression that calculated
/// the weight, and the path on the resource that would contain the result.
pub dynamic_value: Vec<RequestOrchestrationDynamicValue>,
/// Goals that are intended to be achieved by following the requests in this action.
pub goal: Vec<Reference>,
/// Defines the grouping behavior for the action and its children.
pub grouping_behavior: Code,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Defines input data requirements for the action.
pub input: Vec<RequestOrchestrationInput>,
/// The linkId of the action from the PlanDefinition that corresponds to this action
/// in the RequestOrchestration resource.
pub link_id: String,
/// Identifies the facility where the action will occur; e.g. home, hospital,
/// specific clinic, etc.
pub location: CodeableReference,
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
pub modifier_extension: Vec<Extension>,
/// Defines the outputs of the action, if any.
pub output: Vec<RequestOrchestrationOutput>,
/// The participant that should perform or be responsible for this action.
pub participant: Vec<RequestOrchestrationParticipant>,
/// Defines whether the action should usually be preselected.
pub precheck_behavior: Code,
/// A user-visible prefix for the action. For example a section or item numbering
/// such as 1. or A.
pub prefix: String,
/// Indicates how quickly the action should be addressed with respect to other
/// actions.
pub priority: Code,
/// A relationship to another action such as "before" or "30-60 minutes after start
/// of".
pub related_action: Vec<RequestOrchestrationRelatedAction>,
/// Defines expectations around whether an action is required.
pub required_behavior: Code,
/// The resource that is the target of the action (e.g. CommunicationRequest).
pub resource: Reference,
/// Defines the selection behavior for the action and its children.
pub selection_behavior: Code,
/// A text equivalent of the action to be performed. This provides a human-
/// interpretable description of the action when the definition is consumed by a
/// system that might not be capable of interpreting it dynamically.
pub text_equivalent: Markdown,
/// An optional value describing when the action should be performed.
pub timing_age: Age,
/// An optional value describing when the action should be performed.
pub timing_date_time: String,
/// An optional value describing when the action should be performed.
pub timing_duration: Duration,
/// An optional value describing when the action should be performed.
pub timing_period: Period,
/// An optional value describing when the action should be performed.
pub timing_range: Range,
/// An optional value describing when the action should be performed.
pub timing_timing: Timing,
/// The title of the action displayed to a user.
pub title: String,
/// A reference to a StructureMap resource that defines a transform that can be
/// executed to produce the intent resource using the ActivityDefinition instance as
/// the input.
pub transform: Canonical,
/// The type of action to perform (create, update, remove).
pub r#type: CodeableConcept,
}
