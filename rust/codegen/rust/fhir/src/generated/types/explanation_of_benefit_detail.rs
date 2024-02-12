use super::*;
/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug,Clone,PartialEq)]
pub struct ExplanationOfBenefitDetail {
/// The adjudication results.
pub adjudication: Vec<ExplanationOfBenefitAdjudication>,
/// Code to identify the general type of benefits under which products and services
/// are provided.
pub category: CodeableConcept,
/// A real number that represents a multiplier used in determining the overall value
/// of services delivered and/or goods received. The concept of a Factor allows for
/// a discount or surcharge multiplier to be applied to a monetary amount.
pub factor: Decimal,
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
/// The total amount claimed for the group (if a grouper) or the line item.detail.
/// Net = unit price * quantity * factor.
pub net: Money,
/// The numbers associated with notes below which apply to the adjudication of this
/// item.
pub note_number: Vec<PositiveInt>,
/// The amount paid by the patient, in total at the claim claim level or
/// specifically for the item and detail level, to the provider for goods and
/// services.
pub patient_paid: Money,
/// When the value is a group code then this item collects a set of related item
/// details, otherwise this contains the product, service, drug or other billing
/// code for the item. This element may be the start of a range of .productOrService
/// codes used in conjunction with .productOrServiceEnd or it may be a solo element
/// where .productOrServiceEnd is not used.
pub product_or_service: CodeableConcept,
/// This contains the end of a range of product, service, drug or other billing
/// codes for the item. This element is not used when the .productOrService is a
/// group code. This value may only be present when a .productOfService code has
/// been provided to convey the start of the range. Typically this value may be used
/// only with preauthorizations and not with claims.
pub product_or_service_end: CodeableConcept,
/// Identifies the program under which this may be recovered.
pub program_code: Vec<CodeableConcept>,
/// The number of repetitions of a service or product.
pub quantity: Quantity,
/// The type of revenue or cost center providing the product and/or service.
pub revenue: CodeableConcept,
/// The high-level results of the adjudication if adjudication has been performed.
pub review_outcome: ExplanationOfBenefitReviewOutcome,
/// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
/// details which are simple items.
pub sequence: PositiveInt,
/// Third-tier of goods and services.
pub sub_detail: Vec<ExplanationOfBenefitSubDetail>,
/// The total of taxes applicable for this product or service.
pub tax: Money,
/// Trace number for tracking purposes. May be defined at the jurisdiction level or
/// between trading partners.
pub trace_number: Vec<Identifier>,
/// Unique Device Identifiers associated with this line item.
pub udi: Vec<Reference>,
/// If the item is not a group then this is the fee for the product or service,
/// otherwise this is the total of the fees for the details of the group.
pub unit_price: Money,
}
