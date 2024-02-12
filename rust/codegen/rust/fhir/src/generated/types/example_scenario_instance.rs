use super::*;
/// Example of workflow instance.
#[derive(Debug,Clone,PartialEq)]
pub struct ExampleScenarioInstance {
/// References to other instances that can be found within this instance (e.g. the
/// observations contained in a bundle).
pub contained_instance: Vec<ExampleScenarioContainedInstance>,
/// Points to an instance (typically an example) that shows the data that would
/// corespond to this instance.
pub content: Reference,
/// An explanation of what the instance contains and what it's for.
pub description: Markdown,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// A unique string within the scenario that is used to reference the instance.
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
/// Refers to a profile, template or other ruleset the instance adheres to.
pub structure_profile_canonical: String,
/// Refers to a profile, template or other ruleset the instance adheres to.
pub structure_profile_uri: String,
/// A code indicating the kind of data structure (FHIR resource or some other
/// standard) this is an instance of.
pub structure_type: Coding,
/// Conveys the version of the data structure instantiated.  I.e. what release of
/// FHIR, X12, OpenEHR, etc. is instance compliant with.
pub structure_version: String,
/// A short descriptive label the instance to be used in tables or diagrams.
pub title: String,
/// Represents the instance as it was at a specific time-point.
pub version: Vec<ExampleScenarioVersion>,
}
