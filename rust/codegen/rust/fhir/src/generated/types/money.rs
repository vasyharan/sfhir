use super::*;
/// An amount of economic utility in some recognized currency.
#[derive(Debug,Clone,PartialEq)]
pub struct Money {
/// ISO 4217 Currency Code.
pub currency: Code,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Numerical value (with implicit precision).
pub value: Decimal,
}
