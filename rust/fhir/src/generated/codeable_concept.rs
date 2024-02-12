/// A concept that may be defined by a formal reference to a terminology or ontology
/// or may be provided by text.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeableConcept {
    /// A reference to a code defined by a terminology system.
    pub coding: Vec<super::coding::Coding>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A human language representation of the concept as seen/selected/uttered by the
    /// user who entered the data and/or which represents the intended meaning of the
    /// user.
    pub text: super::string::String,
}
