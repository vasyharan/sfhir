/// A relationship of two Quantity values - expressed as a numerator and a
/// denominator.
#[derive(Debug, Clone, PartialEq)]
pub struct Ratio {
    /// The value of the denominator.
    pub denominator: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The value of the numerator.
    pub numerator: super::quantity::Quantity,
}
