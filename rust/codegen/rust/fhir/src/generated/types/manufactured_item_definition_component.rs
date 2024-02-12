use super::*;
/// The definition and characteristics of a medicinal manufactured item, such as a
/// tablet or capsule, as contained in a packaged medicinal product.
#[derive(Debug,Clone,PartialEq)]
pub struct ManufacturedItemDefinitionComponent {
/// The measurable amount of total quantity of all substances in the component,
/// expressable in different ways (e.g. by mass or volume).
pub amount: Vec<Quantity>,
/// A component that this component contains or is made from.
pub component: Vec<ManufacturedItemDefinitionComponent>,
/// A reference to a constituent of the manufactured item as a whole, linked here
/// so that its component location within the item can be indicated. This not where
/// the item's ingredient are primarily stated (for which see Ingredient.for or
/// ManufacturedItemDefinition.ingredient).
pub constituent: Vec<ManufacturedItemDefinitionConstituent>,
/// The function of this component within the item e.g. delivers active ingredient,
/// masks taste.
pub function: Vec<CodeableConcept>,
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
/// General characteristics of this component.
pub property: Vec<ManufacturedItemDefinitionProperty>,
/// Defining type of the component e.g. shell, layer, ink.
pub r#type: CodeableConcept,
}
