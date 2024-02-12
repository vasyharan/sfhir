/// A set of ordered Quantities defined by a low and high limit.
#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    /// The high limit. The boundary is inclusive.
    pub high: super::quantity::Quantity,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The low limit. The boundary is inclusive.
    pub low: super::quantity::Quantity,
}
