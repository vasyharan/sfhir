use super::*;
/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug,Clone,PartialEq)]
pub struct NutritionOrderSupplement {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// Free text or additional instructions or information pertaining to the oral
/// supplement.
pub instruction: String,
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
/// The product or brand name of the nutritional supplement such as "Acme Protein
/// Shake".
pub product_name: String,
/// The amount of the nutritional supplement to be given.
pub quantity: Quantity,
/// Schedule information for a supplement.
pub schedule: NutritionOrderSchedule1,
/// The kind of nutritional supplement product required such as a high protein or
/// pediatric clear liquid supplement.
pub r#type: CodeableReference,
}
