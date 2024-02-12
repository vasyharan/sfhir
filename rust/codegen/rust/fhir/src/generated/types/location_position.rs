use super::*;
/// Details and position information for a place where services are provided and
/// resources and participants may be stored, found, contained, or accommodated.
#[derive(Debug,Clone,PartialEq)]
pub struct LocationPosition {
/// Altitude. The value domain and the interpretation are the same as for the text
/// of the altitude element in KML (see notes on Location main page).
pub altitude: Decimal,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Latitude. The value domain and the interpretation are the same as for the text
/// of the latitude element in KML (see notes on Location main page).
pub latitude: Decimal,
/// Longitude. The value domain and the interpretation are the same as for the text
/// of the longitude element in KML (see notes on Location main page).
pub longitude: Decimal,
/// May be used to represent additional information that is not part of the basic
/// definition of the element and that modifies the understanding of the element
/// in which it is contained and/or the understanding of the containing element's
/// descendants. Usually modifier elements provide negation or qualification.
/// To make the use of extensions safe and managable, there is a strict set
/// of governance applied to the definition and use of extensions. Though any
/// implementer can define an extension, there is a set of requirements that SHALL
/// be met as part of the definition of the extension. Applications processing a
/// resource are required to check for modifier extensions.
///
/// Modifier extensions SHALL NOT change the meaning of any elements on Resource
/// or DomainResource (including cannot change the meaning of modifierExtension
/// itself).
pub modifier_extension: Vec<Extension>,
}
