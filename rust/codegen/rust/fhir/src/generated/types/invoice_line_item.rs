use super::*;
/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
#[derive(Debug,Clone,PartialEq)]
pub struct InvoiceLineItem {
/// The ChargeItem contains information such as the billing code, date, amount etc.
/// If no further details are required for the lineItem, inline billing codes can be
/// added using the CodeableConcept data type instead of the Reference.
pub charge_item_codeable_concept: CodeableConcept,
/// The ChargeItem contains information such as the billing code, date, amount etc.
/// If no further details are required for the lineItem, inline billing codes can be
/// added using the CodeableConcept data type instead of the Reference.
pub charge_item_reference: Reference,
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
/// The price for a ChargeItem may be calculated as a base price with surcharges/
/// deductions that apply in certain conditions. A ChargeItemDefinition resource
/// that defines the prices, factors and conditions that apply to a billing code
/// is currently under development. The priceComponent element can be used to offer
/// transparency to the recipient of the Invoice as to how the prices have been
/// calculated.
pub price_component: Vec<MonetaryComponent>,
/// Sequence in which the items appear on the invoice.
pub sequence: PositiveInt,
/// Date/time(s) range when this service was delivered or completed.
pub serviced_date: String,
/// Date/time(s) range when this service was delivered or completed.
pub serviced_period: Period,
}