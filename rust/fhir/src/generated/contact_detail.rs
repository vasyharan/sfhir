/// Specifies contact information for a person or organization.
#[derive(Debug, Clone, PartialEq)]
pub struct ContactDetail {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The name of an individual to contact.
    pub name: super::string::String,
    /// The contact details for the individual (if a name was provided) or the
    /// organization.
    pub telecom: Vec<super::contact_point::ContactPoint>,
}
