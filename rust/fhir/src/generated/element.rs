/// Base definition for all elements in a resource.
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
}
