use super::*;
/// A date, date-time or partial date (e.g. just year or year + month).  If hours
/// and minutes are specified, a UTC offset SHALL be populated. The format is a
/// union of the schema types gYear, gYearMonth, date and dateTime. Seconds must
/// be provided due to schema type constraints but may be zero-filled and may be
/// ignored.                 Dates SHALL be valid dates.
#[derive(Debug,Clone,PartialEq)]
pub struct DateTime(String);