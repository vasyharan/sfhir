/// A measured amount (or an amount that can potentially be measured). Note that
/// measured amounts include amounts that are not precisely quantified, including
/// amounts involving arbitrary units and floating currencies.
#[derive(Debug, Clone, PartialEq)]
pub struct Count {
    /// A computer processable form of the unit in some unit representation system.
    pub code: super::code::Code,
    /// How the value should be understood and represented - whether the actual value
    /// is greater or less than the stated value due to measurement issues; e.g. if the
    /// comparator is "<" , then the real value is < stated value.
    pub comparator: Comparator,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The identification of the system that provides the coded form of the unit.
    pub system: super::uri::Uri,
    /// A human-readable form of the unit.
    pub unit: super::string::String,
    /// The value of the measured amount. The value includes an implicit precision in
    /// the presentation of the value.
    pub value: super::decimal::Decimal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparator {
    LT,
    LTE,
    GTE,
    GT,
    Ad,
}
