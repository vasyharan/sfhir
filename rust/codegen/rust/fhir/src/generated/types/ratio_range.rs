use super::*;
/// A range of ratios expressed as a low and high numerator and a denominator.
#[derive(Debug,Clone,PartialEq)]
pub struct RatioRange {
/// The value of the denominator.
pub denominator: Quantity,
/// The value of the high limit numerator.
pub high_numerator: Quantity,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The value of the low limit numerator.
pub low_numerator: Quantity,
}
