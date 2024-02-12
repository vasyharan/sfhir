use super::*;
/// A food or supplement that is consumed by patients.
#[derive(Debug,Clone,PartialEq)]
pub struct NutritionProductInstance {
/// An identifier that supports traceability to the event during which material in
/// this product from one or more biological entities was obtained or pooled.
pub biological_source_event: Identifier,
/// The time after which the product is no longer expected to be in proper
/// condition, or its use is not advised or not allowed.
pub expiry: DateTime,
/// Unique id for the element within a resource (for internal references). This may
/// be any string value that does not contain spaces.
pub id: String,
/// The identifier for the physical instance, typically a serial number or
/// manufacturer number.
pub identifier: Vec<Identifier>,
/// The identification of the batch or lot of the product.
pub lot_number: String,
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
/// The name for the specific product.
pub name: String,
/// The amount of items or instances that the resource considers, for instance when
/// referring to 2 identical units together.
pub quantity: Quantity,
/// The time after which the product is no longer expected to be in proper
/// condition, or its use is not advised or not allowed.
pub use_by: DateTime,
}
