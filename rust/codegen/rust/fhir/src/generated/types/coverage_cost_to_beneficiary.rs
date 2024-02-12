use super::*;
/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug,Clone,PartialEq)]
pub struct CoverageCostToBeneficiary {
/// Code to identify the general type of benefits under which products and services
/// are provided.
pub category: CodeableConcept,
/// A suite of codes indicating exceptions or reductions to patient costs and their
/// effective periods.
pub exception: Vec<CoverageException>,
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
/// Is a flag to indicate whether the benefits refer to in-network providers or out-
/// of-network providers.
pub network: CodeableConcept,
/// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
/// annual visits'.
pub term: CodeableConcept,
/// The category of patient centric costs associated with treatment.
pub r#type: CodeableConcept,
/// Indicates if the benefits apply to an individual or to the family.
pub unit: CodeableConcept,
/// The amount due from the patient for the cost category.
pub value_money: Money,
/// The amount due from the patient for the cost category.
pub value_quantity: Quantity,
}
