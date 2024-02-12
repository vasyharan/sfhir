use super::*;
/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug,Clone,PartialEq)]
pub struct PatientLink {
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
/// Link to a Patient or RelatedPerson resource that concerns the same actual
/// individual.
pub other: Reference,
/// The type of link between this patient resource and another patient resource.
pub r#type: Code,
}
