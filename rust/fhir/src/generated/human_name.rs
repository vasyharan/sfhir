/// A name, normally of a human, that can be used for other living entities (e.g.
/// animals but not organizations) that have been assigned names by a human and may
/// need the use of name parts or the need for usage information.
#[derive(Debug, Clone, PartialEq)]
pub struct HumanName {
    /// The part of a name that links to the genealogy. In some cultures (e.g. Eritrea)
    /// the family name of a son is the first name of his father.
    pub family: super::string::String,
    /// Given name.
    pub given: Vec<super::string::String>,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Indicates the period of time when this name was valid for the named person.
    pub period: super::period::Period,
    /// Part of the name that is acquired as a title due to academic, legal, employment
    /// or nobility status, etc. and that appears at the start of the name.
    pub prefix: Vec<super::string::String>,
    /// Part of the name that is acquired as a title due to academic, legal, employment
    /// or nobility status, etc. and that appears at the end of the name.
    pub suffix: Vec<super::string::String>,
    /// Specifies the entire name as it should be displayed e.g. on an application UI.
    /// This may be provided instead of or as well as the specific parts.
    pub text: super::string::String,
    /// Identifies the purpose for this name.
    pub r#use: Use,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Use {
    Usual,
    Official,
    Temp,
    Nickname,
    Anonymous,
    Old,
    Maiden,
}
