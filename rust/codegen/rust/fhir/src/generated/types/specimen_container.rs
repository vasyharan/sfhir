use super::*;
/// A sample to be used for analysis.
#[derive(Debug,Clone,PartialEq)]
pub struct SpecimenContainer {
/// The device resource for the the container holding the specimen. If the container
/// is in a holder then the referenced device will point to a parent device.
pub device: Reference,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The location of the container holding the specimen.
pub location: Reference,
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
/// The quantity of specimen in the container; may be volume, dimensions, or other
/// appropriate measurements, depending on the specimen type.
pub specimen_quantity: Quantity,
}
