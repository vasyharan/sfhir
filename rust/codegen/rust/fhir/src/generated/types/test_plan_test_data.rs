use super::*;
/// A plan for executing testing on an artifact or specifications.
#[derive(Debug,Clone,PartialEq)]
pub struct TestPlanTestData {
/// The actual test resources when they exist.
pub content: Reference,
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
/// Pointer to a definition of test resources - narrative or structured e.g.
/// synthetic data generation, etc.
pub source_reference: Reference,
/// Pointer to a definition of test resources - narrative or structured e.g.
/// synthetic data generation, etc.
pub source_string: String,
/// The type of test data description, e.g. 'synthea'.
pub r#type: Coding,
}
