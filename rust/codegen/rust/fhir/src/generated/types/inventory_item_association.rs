use super::*;
/// A functional description of an inventory item used in inventory and supply-
/// related workflows.
#[derive(Debug,Clone,PartialEq)]
pub struct InventoryItemAssociation {
/// This attribute defined the type of association when establishing associations or
/// relations between items, e.g. 'packaged within' or 'used with' or 'to be mixed
/// with.
pub association_type: CodeableConcept,
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
/// The quantity of the related product in this product - Numerator is the quantity
/// of the related product. Denominator is the quantity of the present product. For
/// example a value of 20 means that this product contains 20 units of the related
/// product; a value of 1:20 means the inverse - that the contained product contains
/// 20 units of the present product.
pub quantity: Ratio,
/// The related item or product.
pub related_item: Reference,
}