use super::*;
/// Example of workflow instance.
#[derive(Debug,Clone,PartialEq)]
pub struct ExampleScenarioStep {
/// Indicates an alternative step that can be taken instead of the sub-process,
/// scenario or operation.  E.g. to represent non-happy-path/exceptional/atypical
/// circumstances.
pub alternative: Vec<ExampleScenarioAlternative>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
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
/// The sequential number of the step, e.g. 1.2.5.
pub number: String,
/// The step represents a single operation invoked on receiver by sender.
pub operation: ExampleScenarioOperation,
/// If true, indicates that, following this step, there is a pause in the flow and
/// the subsequent step will occur at some later time (triggered by some event).
pub pause: Boolean,
/// Indicates that the step is a complex sub-process with its own steps.
pub process: ExampleScenarioProcess,
/// Indicates that the step is defined by a seaparate scenario instance.
pub workflow: Canonical,
}
