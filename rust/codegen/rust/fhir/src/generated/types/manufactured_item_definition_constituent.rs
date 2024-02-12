use super::*;
/// The definition and characteristics of a medicinal manufactured item, such as a
/// tablet or capsule, as contained in a packaged medicinal product.
#[derive(Debug,Clone,PartialEq)]
pub struct ManufacturedItemDefinitionConstituent {
/// The measurable amount of the substance, expressable in different ways (e.g. by
/// mass or volume).
pub amount: Vec<Quantity>,
/// The function of this constituent within the component e.g. binder.
pub function: Vec<CodeableConcept>,
/// The ingredient that is the constituent of the given component.
pub has_ingredient: Vec<CodeableReference>,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The physical location of the constituent/ingredient within the component.
/// Example – if the component is the bead in the capsule, then the location would
/// be where the ingredient resides within the product part – intragranular, extra-
/// granular, etc.
pub location: Vec<CodeableConcept>,
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
