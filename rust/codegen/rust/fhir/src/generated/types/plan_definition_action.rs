use super::*;
/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical and non-clinical artifacts such
/// as clinical decision support rules, order sets, protocols, and drug quality
/// specifications.
#[derive(Debug,Clone,PartialEq)]
pub struct PlanDefinitionAction {
/// Sub actions that are contained within the action. The behavior of this action
/// determines the functionality of the sub-actions. For example, a selection
/// behavior of at-most-one indicates that of the sub-actions, at most one may be
/// chosen as part of realizing the action definition.
pub action: Vec<PlanDefinitionAction>,
/// Defines whether the action can be selected multiple times.
pub cardinality_behavior: Code,
/// A code that provides a meaning, grouping, or classification for the action or
/// action group. For example, a section may have a LOINC code for the section of a
/// documentation template. In pharmaceutical quality, an action (Test) such as pH
/// could be classified as a physical property.
pub code: CodeableConcept,
/// An expression that describes applicability criteria or start/stop conditions for
/// the action.
pub condition: Vec<PlanDefinitionCondition>,
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
pub description: Markdown,
/// Didactic or other informational resources associated with the action that can
/// be provided to the CDS recipient. Information resources can include inline text
/// commentary and links to web resources.
pub documentation: Vec<RelatedArtifact>,
/// Customizations that should be applied to the statically defined resource. For
/// example, if the dosage of a medication must be computed based on the patient's
/// weight, a customization would be used to specify an expression that calculated
/// the weight, and the path on the resource that would contain the result.
pub dynamic_value: Vec<PlanDefinitionDynamicValue>,
/// Identifies goals that this action supports. The reference must be to a goal
/// element defined within this plan definition. In pharmaceutical quality, a goal
/// represents acceptance criteria (Goal) for a given action (Test), so the goalId
/// would be the unique id of a defined goal element establishing the acceptance
/// criteria for the action.
pub goal_id: Vec<Id>,
/// Defines the grouping behavior for the action and its children.
pub grouping_behavior: Code,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Defines input data requirements for the action.
pub input: Vec<PlanDefinitionInput>,
/// An identifier that is unique within the PlanDefinition to allow linkage within
/// the realized CarePlan and/or RequestOrchestration.
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
pub output: Vec<PlanDefinitionOutput>,
/// Indicates who should participate in performing the action described.
pub participant: Vec<PlanDefinitionParticipant>,
/// Defines whether the action should usually be preselected.
pub precheck_behavior: Code,
/// A user-visible prefix for the action. For example a section or item numbering
/// such as 1. or A.
pub prefix: String,
/// Indicates how quickly the action should be addressed with respect to other
/// actions.
pub priority: Code,
/// A description of why this action is necessary or appropriate.
pub reason: Vec<CodeableConcept>,
/// A relationship to another action such as "before" or "30-60 minutes after start
/// of".
pub related_action: Vec<PlanDefinitionRelatedAction>,
/// Defines the required behavior for the action.
pub required_behavior: Code,
/// Defines the selection behavior for the action and its children.
pub selection_behavior: Code,
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
pub subject_codeable_concept: CodeableConcept,
/// A code, group definition, or canonical reference that describes the intended
/// subject of the action and its children, if any. Canonical references are
/// allowed to support the definition of protocols for drug and substance quality
/// specifications, and is allowed to reference a MedicinalProductDefinition,
/// SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition,
/// or PackagedProductDefinition resource.
pub subject_reference: Reference,
/// A text equivalent of the action to be performed. This provides a human-
/// interpretable description of the action when the definition is consumed by a
/// system that might not be capable of interpreting it dynamically.
pub text_equivalent: Markdown,
/// An optional value describing when the action should be performed.
pub timing_age: Age,
/// An optional value describing when the action should be performed.
pub timing_duration: Duration,
/// An optional value describing when the action should be performed.
pub timing_range: Range,
/// An optional value describing when the action should be performed.
pub timing_timing: Timing,
/// The textual description of the action displayed to a user. For example, when the
/// action is a test to be performed, the title would be the title of the test such
/// as Assay by HPLC.
pub title: String,
/// A reference to a StructureMap resource that defines a transform that can be
/// executed to produce the intent resource using the ActivityDefinition instance as
/// the input.
pub transform: Canonical,
/// A description of when the action should be triggered. When multiple triggers are
/// specified on an action, any triggering event invokes the action.
pub trigger: Vec<TriggerDefinition>,
/// The type of action to perform (create, update, remove).
pub r#type: CodeableConcept,
}