/// An address expressed using postal conventions (as opposed to GPS or other
/// location definition formats).  This data type may be used to convey addresses
/// for use in delivering mail as well as for visiting locations which might not be
/// valid for mail delivery.  There are a variety of postal address formats defined
/// around the world.
/// The ISO21090-codedString may be used to provide a coded representation of the
/// contents of strings in an Address.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    /// The name of the city, town, suburb, village or other community or delivery
    /// center.
    pub city: super::string::String,
    /// Country - a nation as commonly understood or generally accepted.
    pub country: super::string::String,
    /// The name of the administrative area (county).
    pub district: super::string::String,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// This component contains the house number, apartment number, street name, street
    /// direction,  P.O. Box number, delivery hints, and similar address information.
    pub line: Vec<super::string::String>,
    /// Time period when address was/is in use.
    pub period: super::period::Period,
    /// A postal code designating a region defined by the postal service.
    pub postal_code: super::string::String,
    /// Sub-unit of a country with limited sovereignty in a federally organized country.
    /// A code may be used if codes are in common use (e.g. US 2 letter state codes).
    pub state: super::string::String,
    /// Specifies the entire address as it should be displayed e.g. on a postal label.
    /// This may be provided instead of or as well as the specific parts.
    pub text: super::string::String,
    /// Distinguishes between physical addresses (those you can visit) and mailing
    /// addresses (e.g. PO Boxes and care-of addresses). Most addresses are both.
    pub r#type: Type,
    /// The purpose of this address.
    pub r#use: Use,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Postal,
    Physical,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Use {
    Home,
    Work,
    Temp,
    Old,
    Billing,
}
