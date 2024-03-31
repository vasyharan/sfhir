/// A date or partial date (e.g. just year or year + month). There is no UTC offset.
/// The format is a union of the schema types gYear, gYearMonth and date.  Dates
/// SHALL be valid dates.
#[derive(Debug, Clone, PartialEq)]
pub struct Date(String);
