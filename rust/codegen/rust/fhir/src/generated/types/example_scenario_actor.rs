use super::*;
/// Example of workflow instance.
#[derive(Debug,Clone,PartialEq)]
pub struct ExampleScenarioActor {
/// An explanation of who/what the actor is and its role in the scenario.
pub description: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A unique string within the scenario that is used to reference the actor.
pub key: String,
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
/// The human-readable name for the actor used when rendering the scenario.
pub title: String,
/// The category of actor - person or system.
pub r#type: Code,
}
