/// A reference to a resource (by instance), or instead, a reference to a concept
/// defined in a terminology or ontology (by class).
#[derive(Debug, Clone, PartialEq)]
pub struct CodeableReference {
    /// A reference to a concept - e.g. the information is identified by its general
    /// class to the degree of precision found in the terminology.
    pub concept: super::codeable_concept::CodeableConcept,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// A reference to a resource the provides exact details about the information being
    /// referenced.
    pub reference: super::reference::Reference,
}
