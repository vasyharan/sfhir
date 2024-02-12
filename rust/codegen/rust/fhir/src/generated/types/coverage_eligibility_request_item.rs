use super::*;
/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug,Clone,PartialEq)]
pub struct CoverageEligibilityRequestItem {
/// Code to identify the general type of benefits under which products and services
/// are provided.
pub category: CodeableConcept,
/// The plan/proposal/order describing the proposed service in detail.
pub detail: Vec<Reference>,
/// Patient diagnosis for which care is sought.
pub diagnosis: Vec<CoverageEligibilityRequestDiagnosis>,
/// Facility where the services will be provided.
pub facility: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Item typification or modifiers codes to convey additional context for the
/// product or service.
pub modifier: Vec<CodeableConcept>,
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
/// This contains the product, service, drug or other billing code for the item.
pub product_or_service: CodeableConcept,
/// The practitioner who is responsible for the product or service to be rendered to
/// the patient.
pub provider: Reference,
/// The number of repetitions of a service or product.
pub quantity: Quantity,
/// Exceptions, special conditions and supporting information applicable for this
/// service or product line.
pub supporting_info_sequence: Vec<PositiveInt>,
/// The amount charged to the patient by the provider for a single unit.
pub unit_price: Money,
}
