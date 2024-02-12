use super::*;
/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug,Clone,PartialEq)]
pub struct CoverageEligibilityRequestInsurance {
/// A business agreement number established between the provider and the insurer for
/// special business processing purposes.
pub business_arrangement: String,
/// Reference to the insurance card level information contained in the Coverage
/// resource. The coverage issuing insurer will use these details to locate the
/// patient's actual coverage within the insurer's information system.
pub coverage: Reference,
/// A flag to indicate that this Coverage is to be used for evaluation of this
/// request when set to true.
pub focal: Boolean,
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
