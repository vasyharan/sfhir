use super::*;
/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation UID or a Segmentation Number, allowing linkage to an Observation
/// Resource or transferring this information along with the ImagingStudy Resource.
#[derive(Debug,Clone,PartialEq)]
pub struct ImagingSelectionInstance {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Each imaging selection instance or frame list might includes an image region,
/// specified by a region type and a set of 2D coordinates.
///        If the parent imagingSelection.instance contains a subset element of type
/// frame, the image region applies to all frames in the subset list.
pub image_region_2_d: Vec<ImagingSelectionImageRegion2D>,
/// Each imaging selection might includes a 3D image region, specified by a region
/// type and a set of 3D coordinates.
pub image_region_3_d: Vec<ImagingSelectionImageRegion3D>,
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
/// The Instance Number for the selected DICOM instance.
pub number: UnsignedInt,
/// The SOP Class UID for the selected DICOM instance.
pub sop_class: Coding,
/// Selected subset of the SOP Instance. The content and format of the subset item
/// is determined by the SOP Class of the selected instance.
///        May be one of:
///        - A list of frame numbers selected from a multiframe SOP Instance.
///        - A list of Content Item Observation UID values selected from a DICOM SR
/// or other structured document SOP Instance.
///        - A list of segment numbers selected from a segmentation SOP Instance.
///        - A list of Region of Interest (ROI) numbers selected from a radiotherapy
/// structure set SOP Instance.
pub subset: Vec<String>,
/// The SOP Instance UID for the selected DICOM instance.
pub uid: Id,
}