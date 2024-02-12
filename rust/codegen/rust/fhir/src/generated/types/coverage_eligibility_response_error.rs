use super::*;
/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug,Clone,PartialEq)]
pub struct CoverageEligibilityResponseError {
/// An error code,from a specified code system, which details why the eligibility
/// check could not be performed.
pub code: CodeableConcept,
/// A [simple subset of FHIRPath](fhirpath.html#simple) limited to element names,
/// repetition indicators and the default child accessor that identifies one of the
/// elements in the resource that caused this issue to be raised.
pub expression: Vec<String>,
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
}
