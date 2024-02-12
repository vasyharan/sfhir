/// A time period defined by a start and end date and optionally time.
#[derive(Debug, Clone, PartialEq)]
pub struct Period {
    /// The end of the period. If the end of the period is missing, it means no end
    /// was known or planned at the time the instance was created. The start may be in
    /// the past, and the end date in the future, which means that period is expected/
    /// planned to end at that time.
    pub end: super::date_time::DateTime,
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub id: super::string::String,
    /// The start of the period. The boundary is inclusive.
    pub start: super::date_time::DateTime,
}
