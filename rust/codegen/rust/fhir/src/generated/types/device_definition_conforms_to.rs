use super::*;
/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug,Clone,PartialEq)]
pub struct DeviceDefinitionConformsTo {
/// Describes the type of the standard, specification, or formal guidance.
pub category: CodeableConcept,
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
/// Standard, regulation, certification, or guidance website, document, or other
/// publication, or similar, supporting the conformance.
pub source: Vec<RelatedArtifact>,
/// Code that identifies the specific standard, specification, protocol, formal
/// guidance, regulation, legislation, or certification scheme to which the device
/// adheres.
pub specification: CodeableConcept,
/// Identifies the specific form or variant of the standard, specification, or
/// formal guidance. This may be a 'version number', release, document edition,
/// publication year, or other label.
pub version: Vec<String>,
}
