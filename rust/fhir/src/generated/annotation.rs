/// A  text note which also  contains information about who made the statement and
/// when.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    /// The individual responsible for making the annotation.
    pub author_reference: super::reference::Reference,
    /// The individual responsible for making the annotation.
    pub author_string: String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The text of the annotation in markdown format.
    pub text: super::markdown::Markdown,
    /// Indicates when this particular annotation was made.
    pub time: super::date_time::DateTime,
}
