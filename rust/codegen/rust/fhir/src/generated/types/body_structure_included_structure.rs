use super::*;
/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug,Clone,PartialEq)]
pub struct BodyStructureIncludedStructure {
/// Body locations in relation to a specific body landmark (tatoo, scar, other body
/// structure).
pub body_landmark_orientation: Vec<BodyStructureBodyLandmarkOrientation>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Code that represents the included structure laterality.
pub laterality: CodeableConcept,
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
/// Code that represents the included structure qualifier.
pub qualifier: Vec<CodeableConcept>,
/// XY or XYZ-coordinate orientation for structure.
pub spatial_reference: Vec<Reference>,
/// Code that represents the included structure.
pub structure: CodeableConcept,
}
