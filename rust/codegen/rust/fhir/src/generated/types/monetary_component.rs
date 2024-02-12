use super::*;
/// Availability data for an {item}.
#[derive(Debug,Clone,PartialEq)]
pub struct MonetaryComponent {
/// Explicit value amount to be used.
pub amount: Money,
/// Codes may be used to differentiate between kinds of taxes, surcharges, discounts
/// etc.
pub code: CodeableConcept,
/// Factor used for calculating this component.
pub factor: Decimal,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// base | surcharge | deduction | discount | tax | informational.
pub r#type: Code,
}
