use super::*;
/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug,Clone,PartialEq)]
pub struct BodyStructureDistanceFromLandmark {
/// An instrument, tool, analyzer, etc. used in the measurement.
pub device: Vec<CodeableReference>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
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
/// The measured distance (e.g., in cm) from a body landmark.
pub value: Vec<Quantity>,
}