/// An identifier - identifies some entity uniquely and unambiguously. Typically
/// this is used for business identifiers.
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    /// Organization that issued/manages the identifier.
    pub assigner: super::reference::Reference,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Time period during which identifier is/was valid for use.
    pub period: super::period::Period,
    /// Establishes the namespace for the value - that is, an absolute URL that
    /// describes a set values that are unique.
    pub system: super::uri::Uri,
    /// A coded type for the identifier that can be used to determine which identifier
    /// to use for a specific purpose.
    pub r#type: super::codeable_concept::CodeableConcept,
    /// The purpose of this identifier.
    pub r#use: Use,
    /// The portion of the identifier typically relevant to the user and which is unique
    /// within the context of the system.
    pub value: super::string::String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Use {
    Usual,
    Official,
    Temp,
    Secondary,
    Old,
}
