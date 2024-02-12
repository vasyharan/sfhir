use super::*;
/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug,Clone,PartialEq)]
pub struct VisionPrescriptionLensSpecification {
/// Power adjustment for multifocal lenses measured in dioptres (0.25 units).
pub add: Decimal,
/// Adjustment for astigmatism measured in integer degrees.
pub axis: Integer,
/// Back curvature measured in millimetres.
pub back_curve: Decimal,
/// Brand recommendations or restrictions.
pub brand: String,
/// Special color or pattern.
pub color: String,
/// Power adjustment for astigmatism measured in dioptres (0.25 units).
pub cylinder: Decimal,
/// Contact lens diameter measured in millimetres.
pub diameter: Decimal,
/// The recommended maximum wear period for the lens.
pub duration: Quantity,
/// The eye for which the lens specification applies.
pub eye: Code,
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
/// Notes for special requirements such as coatings and lens materials.
pub note: Vec<Annotation>,
/// Contact lens power measured in dioptres (0.25 units).
pub power: Decimal,
/// Allows for adjustment on two axis.
pub prism: Vec<VisionPrescriptionPrism>,
/// Identifies the type of vision correction product which is required for the
/// patient.
pub product: CodeableConcept,
/// Lens power measured in dioptres (0.25 units).
pub sphere: Decimal,
}
