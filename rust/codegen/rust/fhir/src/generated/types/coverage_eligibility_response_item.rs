use super::*;
/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug,Clone,PartialEq)]
pub struct CoverageEligibilityResponseItem {
/// A boolean flag indicating whether a preauthorization is required prior to actual
/// service delivery.
pub authorization_required: Boolean,
/// Codes or comments regarding information or actions associated with the
/// preauthorization.
pub authorization_supporting: Vec<CodeableConcept>,
/// A web location for obtaining requirements or descriptive information regarding
/// the preauthorization.
pub authorization_url: Uri,
/// Benefits used to date.
pub benefit: Vec<CoverageEligibilityResponseBenefit>,
/// Code to identify the general type of benefits under which products and services
/// are provided.
pub category: CodeableConcept,
/// A richer description of the benefit or services covered.
pub description: String,
/// True if the indicated class of service is excluded from the plan, missing or
/// False indicates the product or service is included in the coverage.
pub excluded: Boolean,
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
/// A short name or tag for the benefit.
pub name: String,
/// Is a flag to indicate whether the benefits refer to in-network providers or out-
/// of-network providers.
pub network: CodeableConcept,
/// This contains the product, service, drug or other billing code for the item.
pub product_or_service: CodeableConcept,
/// The practitioner who is eligible for the provision of the product or service.
pub provider: Reference,
/// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
/// annual visits'.
pub term: CodeableConcept,
/// Indicates if the benefits apply to an individual or to the family.
pub unit: CodeableConcept,
}
