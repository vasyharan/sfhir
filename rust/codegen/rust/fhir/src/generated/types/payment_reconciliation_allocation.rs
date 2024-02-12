use super::*;
/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.
#[derive(Debug,Clone,PartialEq)]
pub struct PaymentReconciliationAllocation {
/// The Account to which this payment applies, may be completed by the receiver,
/// used for search.
pub account: Reference,
/// The monetary amount allocated from the total payment to the payable.
pub amount: Money,
/// The date from the response resource containing a commitment to pay.
pub date: Date,
/// The Encounter to which this payment applies, may be completed by the receiver,
/// used for search.
pub encounter: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Unique identifier for the current payment item for the referenced payable.
pub identifier: Identifier,
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
/// The party which is receiving the payment.
pub payee: Reference,
/// Unique identifier for the prior payment item for the referenced payable.
pub predecessor: Identifier,
/// A resource, such as a ClaimResponse, which contains a commitment to payment.
pub response: Reference,
/// A reference to the individual who is responsible for inquiries regarding the
/// response and its payment.
pub responsible: Reference,
/// The party which submitted the claim or financial transaction.
pub submitter: Reference,
/// Specific resource to which the payment/adjustment/advance applies.
pub target: Reference,
///  Identifies the claim line item, encounter or other sub-element being paid. Note
/// payment may be partial, that is not match the then outstanding balance or amount
/// incurred.
pub target_item_identifier: Identifier,
///  Identifies the claim line item, encounter or other sub-element being paid. Note
/// payment may be partial, that is not match the then outstanding balance or amount
/// incurred.
pub target_item_positive_int: u64,
///  Identifies the claim line item, encounter or other sub-element being paid. Note
/// payment may be partial, that is not match the then outstanding balance or amount
/// incurred.
pub target_item_string: String,
/// Code to indicate the nature of the payment.
pub r#type: CodeableConcept,
}
