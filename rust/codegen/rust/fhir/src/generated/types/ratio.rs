use super::*;
/// A relationship of two Quantity values - expressed as a numerator and a
/// denominator.
#[derive(Debug,Clone,PartialEq)]
pub struct Ratio {
/// The value of the denominator.
pub denominator: Quantity,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The value of the numerator.
pub numerator: Quantity,
}
