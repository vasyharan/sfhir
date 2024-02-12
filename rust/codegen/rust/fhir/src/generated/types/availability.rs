use super::*;
/// Availability data for an {item}.
#[derive(Debug,Clone,PartialEq)]
pub struct Availability {
/// Times the {item} is available.
pub available_time: Vec<AvailabilityAvailableTime>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Not available during this time due to provided reason.
pub not_available_time: Vec<AvailabilityNotAvailableTime>,
}
