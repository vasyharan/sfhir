use super::*;
/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug,Clone,PartialEq)]
pub struct IngredientManufacturer {
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// An organization that manufactures this ingredient.
pub manufacturer: Reference,
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
/// The way in which this manufacturer is associated with the ingredient. For
/// example whether it is a possible one (others allowed), or an exclusive
/// authorized one for this ingredient. Note that this is not the manufacturing
/// process role.
pub role: Code,
}
