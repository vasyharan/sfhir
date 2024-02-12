use super::*;
/// Example of workflow instance.
#[derive(Debug,Clone,PartialEq)]
pub struct ExampleScenarioOperation {
/// An explanation of what the operation represents and what it does.
pub description: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The system that invokes the action/transmits the data.
pub initiator: String,
/// If false, the initiator is deactivated right after the operation.
pub initiator_active: Boolean,
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
/// The system on which the action is invoked/receives the data.
pub receiver: String,
/// If false, the receiver is deactivated right after the operation.
pub receiver_active: Boolean,
/// A reference to the instance that is transmitted from requester to receiver as
/// part of the invocation of the operation.
pub request: ExampleScenarioContainedInstance,
/// A reference to the instance that is transmitted from receiver to requester as
/// part of the operation's synchronous response (if any).
pub response: ExampleScenarioContainedInstance,
/// A short descriptive label the step to be used in tables or diagrams.
pub title: String,
/// The standardized type of action (FHIR or otherwise).
pub r#type: Coding,
}
