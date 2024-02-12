/// Availability data for an {item}.
#[derive(Debug, Clone, PartialEq)]
pub struct MonetaryComponent {
    /// Explicit value amount to be used.
    pub amount: super::money::Money,
    /// Codes may be used to differentiate between kinds of taxes, surcharges, discounts
    /// etc.
    pub code: super::codeable_concept::CodeableConcept,
    /// Factor used for calculating this component.
    pub factor: super::decimal::Decimal,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// base | surcharge | deduction | discount | tax | informational.
    pub r#type: super::code::Code,
}
