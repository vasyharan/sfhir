use super::*;
/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation UID or a Segmentation Number, allowing linkage to an Observation
/// Resource or transferring this information along with the ImagingStudy Resource.
#[derive(Debug,Clone,PartialEq)]
pub struct ImagingSelectionImageRegion3D {
/// The coordinates describing the image region. Encoded as an ordered set
/// of (x,y,z) triplets (in mm and may be negative) that define a region of
/// interest in the patient-relative Reference Coordinate System defined by
/// ImagingSelection.frameOfReferenceUid element.
pub coordinate: Vec<Decimal>,
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
/// Specifies the type of image region.
pub region_type: Code,
}
