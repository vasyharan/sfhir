/// Details for all kinds of technology mediated contact points for a person or
/// organization, including telephone, email, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct ContactPoint {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// Time period when the contact point was/is in use.
    pub period: super::period::Period,
    /// Specifies a preferred order in which to use a set of contacts. ContactPoints
    /// with lower rank values are more preferred than those with higher rank values.
    pub rank: super::positive_int::PositiveInt,
    /// Telecommunications form for contact point - what communications system is
    /// required to make use of the contact.
    pub system: System,
    /// Identifies the purpose for the contact point.
    pub r#use: Use,
    /// The actual contact point details, in a form that is meaningful to the designated
    /// communication system (i.e. phone number or email address).
    pub value: super::string::String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum System {
    Phone,
    Fax,
    Email,
    Pager,
    Url,
    Sms,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Use {
    Home,
    Work,
    Temp,
    Old,
    Mobile,
}
